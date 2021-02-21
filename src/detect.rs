extern crate crypto;
extern crate swc_common;
use std::collections::HashMap;
use swc_common::BytePos;

use crate::tokenmap::{Clone, CloneLoc, TokenItem, TokenMap};

pub fn detect(
  tokenmap: &mut TokenMap,
  store: &mut HashMap<String, TokenItem>,
  clones: &mut Vec<Clone>,
) {
  let mut saved: Option<BytePos> = None;
  let mut clone: Option<Clone> = None;
  loop {
    let item = tokenmap.next();
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
