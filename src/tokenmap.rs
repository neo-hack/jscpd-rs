extern crate swc_common;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use swc_common::{BytePos, Span};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloneLoc {
  pub source_id: String,
  pub fragement: Option<String>,
  pub lo: BytePos,
  pub hi: BytePos,
}

impl CloneLoc {
  pub fn new(source_id: String, lo: BytePos, hi: BytePos) -> CloneLoc {
    CloneLoc {
      source_id,
      lo,
      hi,
      fragement: None,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clone {
  pub duplication_a: CloneLoc,
  pub duplication_b: CloneLoc,
}

impl Clone {
  pub fn enlarge(&mut self, a_hi: BytePos, b_hi: BytePos) {
    self.duplication_a.hi = a_hi;
    self.duplication_b.hi = b_hi;
  }
  pub fn fragement_a(&mut self, fragement_a: String) {
    self.duplication_a.fragement = Some(fragement_a);
  }
  pub fn fragement_b(&mut self, fragement_b: String) {
    self.duplication_b.fragement = Some(fragement_b);
  }
}

#[derive(Debug, Clone)]
pub struct TokenItemValue {
  pub id: String,
  pub source_id: String,
  pub start: Option<Span>,
  pub end: Option<Span>,
}

impl Default for TokenItemValue {
  fn default() -> TokenItemValue {
    TokenItemValue {
      id: String::from(""),
      source_id: String::from(""),
      start: None,
      end: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct TokenItem {
  pub done: bool,
  pub value: TokenItemValue,
}

impl Default for TokenItem {
  fn default() -> TokenItem {
    TokenItem {
      done: true,
      value: TokenItemValue::default(),
    }
  }
}

pub struct TokenMap {
  pub tokens: std::vec::Vec<swc_ecma_parser::token::TokenAndSpan>,
  pub str: String,
  pub source_id: String,
  pub position: usize,
  pub min_token: usize,
}

impl TokenMap {
  pub fn substring(&self, start: usize, end: usize) -> &str {
    &self.str[start..end]
  }
  // token length
  pub fn size(&self) -> usize {
    self.tokens.len()
  }
  pub fn get(&self, index: usize) -> std::option::Option<&swc_ecma_parser::token::TokenAndSpan> {
    self.tokens.get(index)
  }
  pub fn next(&mut self) -> TokenItem {
    if self.size() == 0 {
      TokenItem::default()
    } else {
      let istart = min(self.position, self.size() - 1);
      let iend = min(self.position + self.min_token, self.size() - 1);
      let start = istart * 32;
      let end = iend * 32;
      let start_loc = match self.get(istart) {
        Some(item) => Some(item.span),
        _ => None,
      };
      let end_loc = match self.get(iend) {
        Some(item) => Some(item.span),
        _ => None,
      };
      let value = TokenItemValue {
        id: self.substring(start, end).to_string(),
        start: start_loc,
        source_id: self.source_id.clone(),
        end: end_loc,
      };
      let mut last_pos = 1;
      if self.size() > self.min_token {
        last_pos = self.size() - self.min_token;
      }
      if self.position < last_pos {
        self.position = self.position + 1;
        TokenItem { done: false, value }
      } else {
        TokenItem { done: true, value }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use crate::tokenmap::TokenMap;

  #[test]
  // next on empty tokens map should work fine
  fn empty_tokens() {
    let mut tokenmap = TokenMap {
      tokens: Vec::new(),
      str: String::from("../examples/javascript/file1.js"),
      position: 0,
      min_token: 50,
      source_id: String::from("../examples/javascript/file1.js"),
    };
    tokenmap.next();
  }
}
