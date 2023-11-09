use rand::{distributions::{weighted::WeightedIndex, Distribution}, thread_rng, Rng, rngs::SmallRng, SeedableRng};

use super::{state_setter::StateSetter, wrappers::state_wrapper::StateWrapper};

/// weighted state setter that uses a rand distribution to poll for a choice
pub struct WeightedSampleSetter {
    state_setters: Vec<Box<dyn StateSetter>>,
    distribution: WeightedIndex<f64>,
    rng: SmallRng,
}

impl WeightedSampleSetter {
    pub fn new(state_setters: Vec<Box<dyn StateSetter>>, weights: Vec<f64>, seed: Option<u64>) -> Self {
        assert!(state_setters.len() == weights.len(), "WeightedSampleSetter requires the argument lengths match");
        let distribution = WeightedIndex::new(&weights).unwrap();
        let seed = match seed {
            Some(seed) => seed,
            None => thread_rng().gen_range(0..10000),
        };
        let rng = SmallRng::seed_from_u64(seed);
        WeightedSampleSetter { state_setters, distribution, rng }
    }
}

impl StateSetter for WeightedSampleSetter {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let choice = self.distribution.sample(&mut self.rng);
        self.state_setters[choice].reset(state_wrapper);
    }

    fn set_seed(&mut self, seed: u64) {
        self.rng = SmallRng::seed_from_u64(seed);
        for state_setter in &mut self.state_setters {
            state_setter.set_seed(seed);
        }
    }
}