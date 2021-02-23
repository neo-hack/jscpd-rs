extern crate crypto;
extern crate swc_common;
use crypto::digest::Digest;
use crypto::md5::Md5;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::cmp::min;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_to_string;
use swc_common::BytePos;

use crate::tokenmap::{Clone, CloneLoc, TokenItem, TokenMap};
use crate::tokensize_with_path;

pub struct Detector {
  stores: HashMap<String, HashMap<String, TokenItem>>,
  md5: Md5,
  pub clones: Vec<Clone>,
  pub min_token: usize,
}

impl Default for Detector {
  fn default() -> Detector {
    Detector {
      md5: Md5::new(),
      stores: HashMap::new(),
      clones: Vec::new(),
      min_token: 50,
    }
  }
}

pub struct DetectorConfig {
  pub min_token: usize,
}

impl Detector {
  pub fn new(config: DetectorConfig) -> Detector {
    Detector {
      md5: Md5::new(),
      stores: HashMap::new(),
      clones: Vec::new(),
      min_token: config.min_token,
      ..Default::default()
    }
  }

  pub fn detect_files(&mut self, cwd: &str) {
    let mut _override_builder = OverrideBuilder::new(cwd);
    _override_builder.add("**/*.ts").unwrap();
    _override_builder.add("**/*.tsx").unwrap();
    _override_builder.add("**/*.jsx").unwrap();
    _override_builder.add("**/*.js").unwrap();
    _override_builder.add("!node_modules").unwrap();
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
                  self.md5.reset();
                  self.md5.input_str(&format!("{:?}", token.token));
                  let hash = self.md5.result_str();
                  str.push_str(&hash);
                }
                let mut tokenmap = TokenMap {
                  tokens,
                  str,
                  position: 0,
                  min_token: self.min_token,
                  source_id: format!("{}", entry.path().display()),
                };
                let ext = entry.path().extension().and_then(OsStr::to_str).unwrap();
                let _store = self.stores.get(ext);
                match _store {
                  Some(_) => (),
                  None => {
                    self.stores.insert(ext.to_string(), HashMap::new());
                  }
                }
                let store = self.stores.get_mut(ext).unwrap();
                detect(&mut tokenmap, store, &mut self.clones);
              }
            };
          }
          Err(err) => println!("ERROR: {}", err),
        }
      }
    };
    self.fragment();
  }
  // slice content into duplication fragment
  fn fragment(&mut self) {
    for c in &mut self.clones {
      let is_valid = c.is_valid();
      if is_valid == true {
        let content_a = read_to_string(c.duplication_a.source_id.clone());
        match content_a {
          Ok(content) => {
            let pos = [c.duplication_a.lo.0 as usize, c.duplication_a.hi.0 as usize];
            let start = pos[0];
            let end = min(pos[1], content.len());
            let subcontent = content.get(start..end);
            if subcontent.is_some() {
              c.fragement_a(subcontent.unwrap().to_string());
            }
          }
          Err(e) => println!("{:?}/{:?}, {}", c.duplication_a.lo, c.duplication_a.hi, e),
        }
        let content_b = read_to_string(c.duplication_b.source_id.clone());
        match content_b {
          Ok(content) => {
            let pos = [c.duplication_b.lo.0 as usize, c.duplication_b.hi.0 as usize];
            let start = pos[0];
            let end = min(pos[1], content.len());
            let subcontent = content.get(start..end);
            if subcontent.is_some() {
              c.fragement_b(subcontent.unwrap().to_string());
            }
          }
          Err(e) => println!("{:?}/{:?}, {}", c.duplication_b.lo, c.duplication_b.hi, e),
        }
      }
    }
  }
}

fn detect(
  tokenmap: &mut TokenMap,
  store: &mut HashMap<String, TokenItem>,
  clones: &mut Vec<Clone>,
) {
  let mut saved: Option<BytePos> = None;
  let mut clone: Option<Clone> = None;
  loop {
    let item = tokenmap.next();
    let skip = item.skip;
    if skip == true {
      break;
    }
    let hi = item.value.end.unwrap().hi;
    let value = store.get(&item.value.id);
    let done = item.done;
    match value {
      // code frame in store
      Some(v) => {
        saved = Some(v.value.end.unwrap().hi);
        match clone {
          // clone found first time
          None => {
            let duplication_a_lo = match v.value.start {
              Some(item) => item.lo,
              _ => BytePos(0),
            };
            let duplication_a_hi = match v.value.end {
              Some(item) => item.hi,
              _ => BytePos(0),
            };
            let duplication_b_lo = match item.value.start {
              Some(item) => item.lo,
              _ => BytePos(0),
            };
            let duplication_b_hi = match item.value.end {
              Some(item) => item.hi,
              _ => BytePos(0),
            };
            let duplication_a = CloneLoc::new(
              v.value.source_id.to_string(),
              duplication_a_lo,
              duplication_a_hi,
            );
            let duplication_b =
              CloneLoc::new(item.value.source_id, duplication_b_lo, duplication_b_hi);
            clone = Some(Clone {
              duplication_a,
              duplication_b,
            });
          }
          Some(_) => (),
        }
      }
      // code frame not in store
      _ => {
        // save clone
        match clone {
          Some(item) => {
            clones.push(item.clone());
          }
          _ => (),
        }
        // empty clone
        clone = None;
        // set value
        store.insert(item.value.id.to_string(), item.clone());
      }
    }
    if done == true {
      // save clone
      match clone {
        Some(item) => {
          clones.push(item.clone());
        }
        _ => (),
      }
      break;
    } else {
      if let Some(ref mut c) = clone {
        if saved.is_some() {
          c.enlarge(saved.unwrap(), hi);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tokenmap::{Clone, CloneLoc};
  use crate::{Detector, DetectorConfig};
  use swc_common::BytePos;

  #[test]
  fn detect_files_should_work() {
    let mut detector = Detector::new(DetectorConfig { min_token: 50 });
    detector.detect_files(&String::from("./"));
    assert_ne!(detector.clones.len(), 0);
  }

  #[test]
  fn overflow_loc_shoule_ignore() {
    let mut detector = Detector::new(DetectorConfig { min_token: 50 });
    let duplication_a = CloneLoc::new(
      String::from("examples/javascript/file_1.js"),
      BytePos(1),
      BytePos(0),
    );
    let duplication_b = CloneLoc::new(
      String::from("examples/javascript/file_1.js"),
      BytePos(1),
      BytePos(0),
    );
    let clone = Clone {
      duplication_a,
      duplication_b,
    };
    detector.clones.push(clone);
    detector.fragment();
  }
  #[test]
  fn outside_loc_shoule_ignore() {
    let mut detector = Detector::new(DetectorConfig { min_token: 50 });
    let duplication_a = CloneLoc::new(
      String::from("examples/javascript/file_1.js"),
      BytePos(1),
      BytePos(10000),
    );
    let duplication_b = CloneLoc::new(
      String::from("examples/javascript/file_1.js"),
      BytePos(1),
      BytePos(10000),
    );
    let clone = Clone {
      duplication_a,
      duplication_b,
    };
    detector.clones.push(clone);
    detector.fragment();
  }

  #[test]
  fn single_inside_loc_shoule_ignore() {
    let mut detector = Detector::new(DetectorConfig { min_token: 50 });
    let duplication_a = CloneLoc::new(
      String::from("examples/javascript/file_1.js"),
      BytePos(0),
      BytePos(1),
    );
    let duplication_b = CloneLoc::new(
      String::from("examples/javascript/file_1.js"),
      BytePos(0),
      BytePos(1),
    );
    let clone = Clone {
      duplication_a,
      duplication_b,
    };
    detector.clones.push(clone);
    detector.fragment();
  }
}
