extern crate crypto;
use clap::{App, Arg};
use crypto::digest::Digest;
use crypto::md5::Md5;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use serde_json;
use std::cmp::min;
use std::collections::HashMap;
use std::format;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::ffi::OsStr;

mod tokenmap;
use tokenmap::{TokenMap, TokenItem, Clone};
mod parse;
use parse::{tokensize_with_path};
mod detect;
use detect::detect;

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
    .about("Detect copy/paste in js/ts files")
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

  let filepath = match matches.value_of("filepath") {
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

  let mut md5 = Md5::new();

  let mut stores: HashMap<String, HashMap<String, TokenItem>> = HashMap::new();
  let mut clones: Vec<Clone> = Vec::new();

  // detect with file content
  // let times = [String::from("function foo() {} function foo() {}"), String::from("function foo() {}")];
  // for time in &times {
  //   let tokens = tokensize_with_str(time.into());
  //   let mut str = String::new();
  //   for token in &tokens {
  //     md5.input_str(&format!("{:?}", token.token));
  //     let hash = md5.result_str();
  //     md5.reset();
  //     str.push_str(&hash);
  //     // println!("Token: {:?}, lo: {:?}, hi: {:?}", token.token, token.span.lo, token.span.hi)
  //   }
  //   let mut tokenmap = TokenMap { tokens, str, position: 0, min_token: 2 };
  //   detect(&mut tokenmap, &mut store, &mut clones);
  // }

  let mut _override_builder = OverrideBuilder::new(cwd);
  _override_builder.add("**/*.ts");
  _override_builder.add("**/*.tsx");
  _override_builder.add("**/*.jsx");
  _override_builder.add("**/*.js");
  _override_builder.add("!node_modules");
  let override_builder = _override_builder.build();
  if let Ok(instance) = override_builder {
    let mut builder = WalkBuilder::new(cwd);
    builder.overrides(instance);
    builder.standard_filters(true);
    let walk = builder.build();
    for result in walk {
      // Each item yielded by the iterator is either a directory entry or an
      // error, so either print the path or the error.
      match result {
        Ok(entry) => {
          if let Some(i) = entry.file_type() {
            if !i.is_dir() {
              let tokens = tokensize_with_path(entry.path());
              let mut str = String::new();
              for token in &tokens {
                md5.reset();
                md5.input_str(&format!("{:?}", token.token));
                let hash = md5.result_str();
                str.push_str(&hash);
              }
              let mut tokenmap = TokenMap {
                tokens,
                str,
                position: 0,
                min_token: min_token.parse().unwrap(),
                source_id: format!("{}", entry.path().display()),
              };
              let ext = entry.path().extension().and_then(OsStr::to_str).unwrap();
              let _store = stores.get(ext);
              match _store {
                Some(_) => (),
                None => {
                  stores.insert(ext.to_string(), HashMap::new());
                },
              }
              let store = stores.get_mut(ext).unwrap();
              detect(&mut tokenmap, store, &mut clones);
            }
          };
        }
        Err(err) => println!("ERROR: {}", err),
      }
    }
  }

  for c in &mut clones {
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

  save(&clones);
}
