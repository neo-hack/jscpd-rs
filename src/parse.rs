extern crate swc_common;
extern crate swc_ecma_parser;
use std::path::Path;
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
   FileName, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, EsConfig, Parser, StringInput, Syntax, TsConfig};

#[allow(dead_code)]
pub fn tokensize_with_str(input: String) -> std::vec::Vec<swc_ecma_parser::token::TokenAndSpan> {
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
  let fm = cm.new_source_file(FileName::Custom("test.js".into()), input);

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

pub fn tokensize_with_path(filepath: &Path) -> std::vec::Vec<swc_ecma_parser::token::TokenAndSpan> {
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  let fm = cm
    .load_file(filepath)
    .expect(&format!("failed to load {}", filepath.display()));

  let lexer;

  if filepath.ends_with("jsx") || filepath.ends_with("js") {
    lexer = Lexer::new(
      Syntax::Es(EsConfig {
        jsx: true,
        class_private_props: true,
        class_private_methods: true,
        class_props: true,
        fn_bind: true,
        decorators: true,
        decorators_before_export: true,
        export_default_from: true,
        dynamic_import: true,
        export_namespace_from: true,
        import_assertions: true,
        import_meta: true,
        top_level_await: true,
        nullish_coalescing: false,
        num_sep: false,
        optional_chaining: false,
      }),
      Default::default(),
      StringInput::from(&*fm),
      None,
    );
  } else {
    lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        tsx: true,
        dts: false,
        decorators: true,
        dynamic_import: true,
        import_assertions: true,
        no_early_errors: true,
      }),
      Default::default(),
      StringInput::from(&*fm),
      None,
    );
  }

  let capturing = Capturing::new(lexer);

  let mut parser = Parser::new_from(capturing);

  for e in parser.take_errors() {
    e.into_diagnostic(&handler).emit();
  }

  let _module = parser
    .parse_module()
    .map_err(|e| e.into_diagnostic(&handler).emit());

  let tokens = parser.input().take();
  tokens
}

#[cfg(test)]
mod tests {
  use crate::tokensize_with_path;
  use std::path::Path;

  #[test]
  // parse unsupport syntax should work fine
  fn unsupport_syntax() {
    let tokens = tokensize_with_path(Path::new("examples/javascript/error_typescript.ts"));
    assert_ne!(tokens.len(), 0);
  }

  #[test]
  // parse empty file should return empty vec
  fn empty_file() {
    let tokens = tokensize_with_path(Path::new("examples/javascript/empty_file.ts"));
    assert_eq!(tokens.len(), 0);
  }

  #[test]
  // parse js file should work
  fn parse_js() {
    let tokens = tokensize_with_path(Path::new("examples/javascript/file_1.js"));
    assert_ne!(tokens.len(), 0);
  }

  #[test]
  // parse jsx file should work
  fn parse_jsx() {
    let tokens = tokensize_with_path(Path::new("examples/jsx/file1.jsx"));
    assert_ne!(tokens.len(), 0);
  }

  #[test]
  // parse ts file should work
  fn parse_ts() {
    let tokens = tokensize_with_path(Path::new("examples/javascript/lohi.ts"));
    assert_ne!(tokens.len(), 0);
  }

  #[test]
  // parse tsx file should work
  fn parse_tsx() {
    let tokens = tokensize_with_path(Path::new("examples/jsx/file1.tsx"));
    assert_ne!(tokens.len(), 0);
  }
}
