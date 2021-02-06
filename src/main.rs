// #[doc = "Outer comment"]
// fn main() {
//   println!("Hello, world!");
//   let x: i64 = 1;
//   println!("{}", x)
// }


extern crate swc_common;
extern crate swc_ecma_parser;
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
  FileName, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

fn main() {
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  // Real usage
  // let fm = cm
  //     .load_file(Path::new("test.js"))
  //     .expect("failed to load test.js");

  let fm = cm.new_source_file(
      FileName::Custom("test.js".into()),
      "function foo() {}".into(),
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

  println!("Tokens: {:?}", parser.input().take());
}
