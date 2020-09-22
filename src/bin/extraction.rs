use anyhow::{Context, Result};
use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Params {
    #[structopt(short)]
    n: usize,

    #[structopt(name = "MEMBERS_FILE", parse(from_os_str))]
    path: PathBuf,
}

fn build_rng() -> Result<StdRng> {
    let rng;
    match std::env::var_os("RSPRIZE_SEED") {
        Some(seed) => {
            let seed = seed.into_string().unwrap().parse()?;
            rng = rand::rngs::StdRng::seed_from_u64(seed);
        }
        None => {
            rng = rand::rngs::StdRng::from_rng(rand::thread_rng())?;
        }
    }
    Ok(rng)
}

pub fn main() -> Result<()> {
    let params = Params::from_args();

    let mut members = members::read_accepted_rsvp(
        std::fs::File::open(params.path).context("Cannot open input file")?,
    )
    .context("Extract members")?;

    let mut rng = build_rng()?;

    members.sort_by(|x, y| x.name.cmp(&y.name));
    members.shuffle(&mut rng);

    for (pos, m) in members[0..params.n.min(members.len())]
        .into_iter()
        .enumerate()
    {
        println!("Prize {}: {} Win!", pos + 1, m.name);
    }
    Ok(())
}
