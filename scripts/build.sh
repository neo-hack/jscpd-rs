# macos
cargo build --release 
tar -czf target/release/jscpdrs.tar.gz target/release/jscpdrs
shasum -a 256 target/release/jscpdrs.tar.gz

# macos x86_64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
tar -czf target/x86_64-apple-darwin/release/jscpdrs-x86_64-apple-darwin.tar.gz target/x86_64-apple-darwin/release/jscpdrs