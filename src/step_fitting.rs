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
    pub fn vals(&self, road_weight: f64) -> Vec<f64> {
        let count = self.samples.iter().map(|s| s.len()).sum::<usize>();
        let sum = self.samples.iter().flatten().sum::<f64>();
        let sq_sum = self.samples.iter().flatten().map(|s| s * s).sum::<f64>();
        let mut front_count = 0;
        let mut front_sum = 0.0;
        let mut front_sq_sum = 0.0;

        let (mid, _residual, front_avg, back_avg) = self.samples[..self.samples.len() - 1]
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

                let residual = (front_residual + back_residual) / count as f64;

                (i, residual, front_avg, back_avg)
            })
            .filter(|&(_i, residual, _front_avg, _back_avg)| residual.is_finite())
            .min_by_key(|&(_i, residual, _front_avg, _back_avg)| AnywayOrd(residual))
            .unwrap_or((14, std::f64::NAN, 5000.0, 5000.0));

        let mut ret = vec![back_avg; self.samples.len()];
        for ret in ret[..=mid].iter_mut() {
            *ret = front_avg;
        }
        for (ret, sample) in ret.iter_mut().zip(self.samples.iter()) {
            let sample_avg = if sample.is_empty() {
                5000.0
            } else {
                sample.iter().sum::<f64>() / sample.len() as f64
            };

            *ret = (sample_avg + road_weight * *ret) / (1.0 + road_weight);
            *ret = ret.max(0.0);
        }
        ret
    }
}
