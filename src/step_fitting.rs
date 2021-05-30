extern crate anyway_ord;

use anyway_ord::AnywayOrd;

#[derive(Clone)]
pub struct StepFitting {
    samples: Vec<Vec<f64>>,
}

impl StepFitting {
    pub fn new(len: usize) -> StepFitting {
        Self {
            samples: vec![vec![]; len],
        }
    }
    pub fn add(&mut self, index: usize, sample: f64) {
        self.samples[index].push(sample);
    }
    pub fn clear(&mut self) {
        self.samples.iter_mut().for_each(Vec::clear);
    }
    pub fn vals(&self) -> Vec<f64> {
        let count = self.samples.iter().map(|s| s.len()).sum::<usize>();
        let sum = self.samples.iter().flatten().sum::<f64>();
        let sq_sum = self.samples.iter().flatten().map(|s| s * s).sum::<f64>();
        let mut front_count = 0;
        let mut front_sum = 0.0;
        let mut front_sq_sum = 0.0;

        let (mid, _residual, front_avg, back_avg) = self
            .samples
            .iter()
            .enumerate()
            .map(|(i, samples)| {
                front_count += samples.len();
                front_sum += samples.iter().sum::<f64>();
                front_sq_sum += samples.iter().map(|s| s * s).sum::<f64>();

                let back_count = count - front_count;
                let back_sum = sum - front_sum;
                let back_sq_sum = sq_sum - front_sq_sum;

                let front_avg = front_sum / front_count as f64;
                let back_avg = back_sum / back_count as f64;

                let front_var = front_sq_sum / front_count as f64 - front_avg.powi(2);
                let back_var = back_sq_sum / back_count as f64 - back_avg.powi(2);

                let front_residual = front_var * front_count as f64;
                let back_residual = back_var * back_count as f64;

                let residual = front_residual + back_residual;

                (i, residual, front_avg, back_avg)
            })
            .filter(|&(_i, residual, _front_avg, _back_avg)| residual.is_finite())
            .min_by_key(|&(_i, residual, _front_avg, _back_avg)| AnywayOrd(residual))
            .unwrap_or((0, std::f64::NAN, 5000.0, 5000.0));

        let front = std::iter::repeat(front_avg).take(mid + 1);
        let back = std::iter::repeat(back_avg).take(self.samples.len() - (mid + 1));

        let mut ret = front.chain(back).map(|v| v.max(0.0)).collect::<Vec<_>>();

        for (ret, sample) in ret.iter_mut().zip(self.samples.iter()) {
            if sample.is_empty() {
                *ret = (*ret + 3000.0) / 2.0;
            } else {
                let r = sample.len() as f64;
                let avg = sample.iter().sum::<f64>() / sample.len() as f64;
                *ret = (*ret + avg * r) / (1.0 + r);
            }
        }

        ret
    }
}
