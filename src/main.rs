extern crate crypto;
use clap::{App, Arg};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::process;

mod tokenmap;
use tokenmap::Clone;
mod parse;
use parse::tokensize_with_path;
mod detect;
use detect::{Detector, DetectorConfig};

fn save(clones: &Vec<Clone>) -> std::io::Result<()> {
  let content = serde_json::to_string_pretty(clones)?;
  let mut file = File::create("result.json")?;
  file.write_all(content.as_bytes())?;
  Ok(())
}

fn main() {
  let matches = App::new("jscpdrs")
    .version("0.1.0")
    .author("Jiangweixian. <Jiangweixian1994@gmail.com>")
    .about("Detect copy/paste in js/jsx/ts/tsx files")
    .arg(
      Arg::new("filepath")
        .short('f')
        .about("Sets a detch file")
        .required(false),
    )
    .arg(
      Arg::new("cwd")
        .short('c')
        .long("cwd")
        .about("Sets root path")
        .required(false)
        .default_value("./"),
    )
    .arg(
      Arg::new("min_token")
        .short('m')
        .long("min_token")
        .about("Sets min tokens")
        .default_value("50"),
    )
    .arg(
      Arg::new("ignore")
        .long("ignore")
        .multiple_values(true)
        .about("Sets ignore files pattern")
        .required(false),
    )
    .get_matches();

  let _filepath = match matches.value_of("filepath") {
    Some(f) => f,
    _ => "",
  };

  let min_token = match matches.value_of("min_token") {
    Some(f) => f,
    _ => "50",
  };

  let cwd = match matches.value_of("cwd") {
    Some(f) => f,
    _ => "./",
  };

  let ignore: Vec<String> = matches
    .values_of("ignore")
    .unwrap_or_default()
    .map(|s| s.to_string())
    .collect();

  let mut detector = Detector::new(DetectorConfig {
    min_token: min_token.parse().unwrap(),
    ignore,
  });
  detector.detect_files(cwd);

  save(&detector.clones).unwrap();
  process::exit(0);
}
