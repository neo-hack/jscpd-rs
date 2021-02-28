mkdir builds

# macos
cargo build --release 
mv target/release/jscpdrs builds/jscpdrs
cd builds
tar -czf jscpdrs.tar.gz jscpdrs
shasum -a 256 jscpdrs.tar.gz
cd ..

echo "$PWD"

# macos x86_64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
mkdir builds/jscpdrs-cli-x86_64-apple-darwin
mv target/x86_64-apple-darwin/release/jscpdrs builds/jscpdrs-cli-x86_64-apple-darwin/jscpdrs-cli
cd builds
tar -czf jscpdrs-x86_64-apple-darwin.tar.gz jscpdrs-cli-x86_64-apple-darwin/jscpdrs-cli
cd ..