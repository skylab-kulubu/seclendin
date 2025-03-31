export $(grep -v '^#' .env | xargs)
cargo build --release --target x86_64-pc-windows-gnu
