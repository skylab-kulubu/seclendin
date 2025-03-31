export $(grep -v '^#' .env | xargs)
export RELEASE_TAG=$(curl -s https://api.github.com/repos/skylab-kulubu/seclendin/tags | jq -r '.[0].name') # Latest tag
cargo build --release --target x86_64-pc-windows-gnu
