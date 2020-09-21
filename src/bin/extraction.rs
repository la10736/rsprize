use anyhow::{Context, Result};
use std::path::PathBuf;

use rand::prelude::SliceRandom;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Parameters {
    // Number of extraction (default 1)
    #[structopt(short = "n")]
    extractions: Option<usize>,

    /// Members json file
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<()> {
    let parameters = Parameters::from_args();
    let mut members = members::read_accepted_rsvp(std::fs::File::open(parameters.file)?)?;

    let mut rng = rsprize::rng::build().context("Cannot create random generator")?;

    members.sort_by_key(|m| m.name.clone());
    members.shuffle(&mut rng);

    for (pos, member) in members[..parameters.extractions.unwrap_or(1)]
        .iter()
        .enumerate()
    {
        println!("Prize {}: {} Win!", pos + 1, member.name);
    }

    Ok(())
}
