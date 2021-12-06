#![feature(drain_filter)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

#[macro_use]
extern crate if_chain;

use anyhow::{Context, Result};
use clap::{crate_authors, crate_version, Parser};
use std::{env, fs, path::PathBuf, process::exit, time::Instant};

mod day1;
mod day2;
mod day3;
mod day4;

#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(long, short = 'f', about = "Specifiy another input file to use")]
    input_file: Option<PathBuf>,

    #[clap(about = "What day to solve")]
    day: usize,
    #[clap(about = "What part to solve")]
    part: usize,
}

fn main() -> Result<()> {
    // Parse cli
    let opts = Opts::parse();

    // Init logging (default to info)
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    // Basic bound checks
    if opts.day < 1 || opts.day > 31 {
        error!("Invalid day \"{}\" Only 1-31 are allowed!", opts.day);
        exit(1);
    }
    if opts.part != 1 && opts.part != 2 {
        error!("Invalid part \"{}\" Only 1 and 2 are allowed!", opts.part);
        exit(1);
    }

    let filename = opts
        .input_file
        .unwrap_or(PathBuf::from(format!("../input/day{}.txt", opts.day)));
    let filecontent = fs::read_to_string(filename).context("Reading input file")?;

    let start = Instant::now();
    let solution: String = match opts.day {
        1 => match opts.part {
            1 => day1::solve_part_1(&filecontent).map(|v| v.to_string()),
            2 => day1::solve_part_2(&filecontent).map(|v| v.to_string()),
            _ => unreachable!(),
        },
        2 => match opts.part {
            1 => day2::solve_part_1(&filecontent).map(|v| v.to_string()),
            2 => day2::solve_part_2(&filecontent).map(|v| v.to_string()),
            _ => unreachable!(),
        },
        3 => match opts.part {
            1 => day3::solve_part_1(&filecontent).map(|v| v.to_string()),
            2 => day3::solve_part_2(&filecontent).map(|v| v.to_string()),
            _ => unreachable!(),
        },
        4 => match opts.part {
            1 => day4::solve_part_1(&filecontent).map(|v| v.to_string()),
            2 => day4::solve_part_2(&filecontent).map(|v| v.to_string()),
            _ => unreachable!(),
        },
        _ => {
            unimplemented!()
        }
    }
    .with_context(|| format!("Solving day {} part {}", opts.day, opts.part))?;

    info!("Solved in {:?}: {}", start.elapsed(), solution);
    Ok(())
}
