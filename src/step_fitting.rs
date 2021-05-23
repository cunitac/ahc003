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
    pub fn vals(&self) -> Vec<f64> {
        let count = self.samples.iter().map(|s| s.len()).sum::<usize>();
        let sum = self.samples.iter().flatten().sum::<f64>();
        let sq_sum = self.samples.iter().flatten().map(|s| s * s).sum::<f64>();
        let mut front_count = 0;
        let mut front_sum = 0.0;
        let mut front_sq_sum = 0.0;

        let (mid, _var_sum, front_avg, back_avg) = self.samples[..self.samples.len() - 1]
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

                (i, front_var + back_var, front_avg, back_avg)
            })
            .filter(|&(_i, var_sum, _front_avg, _back_avg)| var_sum.is_finite())
            .min_by_key(|&(_i, var_sum, _front_avg, _back_avg)| AnywayOrd(var_sum))
            .unwrap_or((14, 0.0, 4000.0, 4000.0));
        let mut ret = vec![back_avg; self.samples.len()];
        for ret in ret[..=mid].iter_mut() {
            *ret = front_avg;
        }
        ret
    }
}
