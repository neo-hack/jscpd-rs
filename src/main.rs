extern crate crypto;
use clap::{App, Arg};
use serde_json;
use std::cmp::min;
use std::fs::{read_to_string, File};
use std::io::prelude::*;

mod tokenmap;
use tokenmap::Clone;
mod parse;
use parse::tokensize_with_path;
mod detect;
use detect::{Detector, DetectorConfig};

fn save(clones: &Vec<Clone>) -> std::io::Result<()> {
  let content = serde_json::to_string(clones)?;
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

  let mut detector = Detector::new(DetectorConfig {
    min_token: min_token.parse().unwrap(),
  });
  detector.detect_files(cwd);

  for c in &mut detector.clones {
    let content_a = read_to_string(c.duplication_a.source_id.clone());
    match content_a {
      Ok(content) => {
        let pos = [c.duplication_a.lo.0 as usize, c.duplication_a.hi.0 as usize];
        if pos[0] <= pos[1] {
          let start = pos[0];
          let end = min(pos[1], content.len());
          let subcontent = &content[start..end];
          c.fragement_a(subcontent.to_string());
        } else {
          println!(
            "duplication a {:?}/{:?} {:?}/{:?}",
            c.duplication_a.source_id,
            c.duplication_b.source_id,
            c.duplication_a.lo,
            c.duplication_a.hi
          );
        }
      }
      Err(e) => println!("{:?}/{:?}, {}", c.duplication_a.lo, c.duplication_a.hi, e),
    }
    let content_b = read_to_string(c.duplication_b.source_id.clone());
    match content_b {
      Ok(content) => {
        let pos = [c.duplication_b.lo.0 as usize, c.duplication_b.hi.0 as usize];
        if pos[0] <= pos[1] {
          let start = pos[0];
          let end = min(pos[1], content.len());
          let subcontent = &content[start..end];
          c.fragement_b(subcontent.to_string());
        } else {
          println!(
            "duplication b {:?}/{:?} {:?}/{:?}",
            c.duplication_a.source_id,
            c.duplication_b.source_id,
            c.duplication_b.lo,
            c.duplication_b.hi
          );
        }
      }
      Err(e) => println!("{:?}/{:?}, {}", c.duplication_b.lo, c.duplication_b.hi, e),
    }
  }

  save(&detector.clones).unwrap();
}
