export $(grep -v '^#' .env | xargs)
10039 cargo build --release --target x86_64-pc-windows-gnu
