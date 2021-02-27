cargo build --release
cd target/release
tar -czf jscpdrs.tar.gz jscpdrs
shasum -a 256 jscpdrs.tar.gz
cp target/release/jscpdrs packages/jscpdrs-cli/bin