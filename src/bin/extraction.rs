use anyhow::{Context, Result};
use rand::prelude::SliceRandom;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Params {
    #[structopt(short)]
    n: usize,

    #[structopt(name = "MEMBERS_FILE", parse(from_os_str))]
    path: PathBuf,
}

pub fn main() -> Result<()> {
    let params = Params::from_args();

    let mut members = members::read_accepted_rsvp(
        std::fs::File::open(params.path).context("Cannot open input file")?,
    )
    .context("Extract members")?;

    let mut rng = rsprize::rng::build()?;

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
