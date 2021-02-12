extern crate swc_common;
extern crate swc_ecma_parser;
extern crate crypto;
use std::cmp::min;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::format;
use std::collections::HashMap;
use std::path::Path;
use clap::{Arg, App};
use ignore::{WalkBuilder, Walk};
use ignore::overrides::{Override, OverrideBuilder};
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
  FileName, SourceMap,
  Span, BytePos
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

#[derive(Debug, Clone, Copy)]
struct CloneLoc {
  lo: BytePos,
  hi: BytePos,
}

#[derive(Debug, Clone, Copy)]
struct Clone {
  duplication_a: CloneLoc,
  duplication_b: CloneLoc
}

impl Clone {
  fn enlarge(&mut self, a_hi: BytePos, b_hi: BytePos) {
    self.duplication_a.hi = a_hi;
    self.duplication_b.hi = b_hi;
  }
}

#[derive(Debug, Clone)]
struct TokenItemValue {
  id: String,
  start: Option<Span>,
  end: Option<Span>,
}

#[derive(Debug, Clone)]
struct TokenItem {
  done: bool,
  value: TokenItemValue
}

struct TokenMap {
  tokens: std::vec::Vec<swc_ecma_parser::token::TokenAndSpan>,
  str: String,
  position: usize,
  min_token: usize
}

impl TokenMap {
  fn console(&self) {
    println!("Tokens: {:?}", self.str);
  }
  fn substring(&self, start: usize, end: usize) -> &str {
    &self.str[start..end]
  }
  // token length
  fn size(&self) -> usize {
    self.tokens.len()
  }
  // token hash map length
  fn len(&self) -> usize {
    self.str.len()
  }
  fn get(&self, index: usize) -> std::option::Option<&swc_ecma_parser::token::TokenAndSpan> {
    self.tokens.get(index)
  }
  fn next(&mut self) -> TokenItem {
    let istart = min(self.position, self.size() - 1);
    let iend = min(self.position + self.min_token, self.size() - 1);
    println!("{},{}", istart, iend);
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
      end: end_loc
    };
    if self.position < self.size() - 2 {
      self.position = self.position + 1;
      TokenItem {
        done: false,
        value,
      }
    } else {
      TokenItem {
        done: true,
        value,
      }
    }
  }
}

fn detect(tokenmap: &mut TokenMap, store: &mut HashMap<String, TokenItem>, clones: &mut Vec<Clone>) {
  let mut saved: Option<CloneLoc> = None;
  let mut clone: Option<Clone> = None;
  loop {
    let item = tokenmap.next();
    let hi = item.value.end.unwrap().hi;
    let key = store.get(&item.value.id);
    let done = item.done;
    match key {
      // code frame in store
      Some(v) => {
        let lo = match v.value.start {
          Some(item) => {
            item.lo
          },
          _ => BytePos(0),
        };
        let hi = match v.value.end {
          Some(item) => {
            item.hi
          },
          _ => BytePos(0),
        };
        saved = Some(CloneLoc { lo, hi });
        match clone {
          // clone found first time
          None => {
            let duplication_a_lo = match v.value.start {
              Some(item) => {
                println!("Duplication a clone found {:?}", item.lo);
                item.lo
              },
              _ => BytePos(0),
            };
            let duplication_a_hi = match v.value.end {
              Some(item) => {
                println!("Duplication a clone found {:?}", item.hi);
                item.hi
              },
              _ => BytePos(0),
            };
            let duplication_b_lo = match item.value.start {
              Some(item) => {
                println!("Duplication b clone found {:?}", item.lo);
                item.lo
              },
              _ => BytePos(0),
            };
            let duplication_b_hi = match item.value.end {
              Some(item) => {
                println!("Duplication b clone found {:?}", item.hi);
                item.hi
              },
              _ => BytePos(0),
            };
            let duplication_a = CloneLoc { lo: duplication_a_lo, hi: duplication_a_hi };
            let duplication_b = CloneLoc { lo: duplication_b_lo, hi: duplication_b_hi };
            clone = Some(Clone { duplication_a, duplication_b });
          },
          Some(_) => (),
        }
      }
      // code frame not in store
      _ => {
        // save clone
        match clone {
          Some(item) => {
            clones.push(item);
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
          clones.push(item);
        }
        _ => (),
      };
      break
    } else {
      if let Some(ref mut c) = clone {
        c.enlarge(saved.unwrap().hi, hi);
      }
    }
    println!("Enlarge clone {:?}/{:?}", clone, saved);
  }
}

fn tokensize_with_str(input: String) -> std::vec::Vec<swc_ecma_parser::token::TokenAndSpan> {
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
  let fm = cm.new_source_file(
    FileName::Custom("test.js".into()),
    input,
  );

  let lexer = Lexer::new(
      Syntax::Es(Default::default()),
      Default::default(),
      StringInput::from(&*fm),
      None,
  );

  let capturing = Capturing::new(lexer);

  let mut parser = Parser::new_from(capturing);

  for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
  }

  let _module = parser
      .parse_module()
      .map_err(|e| e.into_diagnostic(&handler).emit())
      .expect("Failed to parse module.");

  let tokens = parser.input().take();
  tokens
}

fn tokensize_with_path(filepath: &Path) -> std::vec::Vec<swc_ecma_parser::token::TokenAndSpan> {
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  let fm = cm
      .load_file(filepath)
      .expect("failed to load test.js");

  let lexer = Lexer::new(
      Syntax::Typescript(Default::default()),
      Default::default(),
      StringInput::from(&*fm),
      None,
  );

  let capturing = Capturing::new(lexer);

  let mut parser = Parser::new_from(capturing);

  for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
  }

  let _module = parser
      .parse_module()
      .map_err(|e| e.into_diagnostic(&handler).emit())
      .expect("Failed to parse module.");

  let tokens = parser.input().take();
  tokens
}

fn main() {
  let matches = App::new("myapp")
        .version("1.0")
        .author("Jiangweixian. <Jiangweixian1994@gmail.com>")
        .about("Does awesome things")
        .arg("-f, --filepath=[FILE] 'Sets a detect file'")
        .arg(Arg::new("min_token").short('m').long("min_token").about("Sets min tokens").default_value("2"))
        .get_matches();

  let filepath = match matches.value_of("filepath") {
    Some(f) => f,
    _ => "",
  };

  let min_token = match matches.value_of("min_token") {
    Some(f) => f,
    _ => "",
  };

  let mut md5 = Md5::new();

  let mut store: HashMap<String, TokenItem> = HashMap::new();
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

  let mut _override_builder = OverrideBuilder::new("./");
  _override_builder.add("**/*.ts");
  _override_builder.add("!node_modules");
  let override_builder = _override_builder.build();
  if let Ok(instance) = override_builder {
    let mut builder = WalkBuilder::new("./");
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
                println!("file {}", entry.path().display());
                let tokens = tokensize_with_path(entry.path());
                let mut str = String::new();
                for token in &tokens {
                  md5.input_str(&format!("{:?}", token.token));
                  let hash = md5.result_str();
                  md5.reset();
                  str.push_str(&hash);
                  // println!("Token: {:?}, lo: {:?}, hi: {:?}", token.token, token.span.lo, token.span.hi)
                }
                let mut tokenmap = TokenMap { tokens, str, position: 0, min_token: min_token.parse().unwrap() };
                detect(&mut tokenmap, &mut store, &mut clones);
              }
            };
          },
          Err(err) => println!("ERROR: {}", err),
      }
    }
  }

  println!("{}", clones.len());

  for c in &clones {
    println!("found duplication a {:?} {:?}", c.duplication_a.lo, c.duplication_a.hi);
    println!("found duplication b {:?} {:?}", c.duplication_b.lo, c.duplication_b.hi);
  }


  // println!("Token: {:?}", tokenmap.substring(0, 2));
  // println!("Token: {:?}", tokenmap.get(0));
  // println!("Token: {:?}", tokenmap.next().value.id);
  // println!("Token: {:?}", tokenmap.position);
  // tokenmap.console();
}
