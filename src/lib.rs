pub mod rng {
    use anyhow::Result;
    use rand::{prelude::StdRng, SeedableRng};

    pub fn build() -> Result<StdRng> {
        let rng;

        match std::env::var_os("RSPRIZE_SEED") {
            Some(seed) => {
                let seed: u64 = seed.into_string().unwrap().parse()?;
                rng = rand::rngs::StdRng::seed_from_u64(seed);
            }
            None => rng = rand::rngs::StdRng::from_rng(rand::thread_rng())?,
        }

        return Ok(rng);
    }
}
