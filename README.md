## Building

```bash
env $(cat .env | xargs) cargo build --target x86_64-pc-windows-gnu --release
```
