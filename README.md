# Compiling on Windows (WSL2)
 Open WSL 2 terminal and install rust via rustup -  Reference: https://rustup.rs/
 ```
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

Clone project:
```
git clone git@github.com:borgmanJeremy/rss.git
```

Build project in release mode
```
cd rss
cargo build --release
```
