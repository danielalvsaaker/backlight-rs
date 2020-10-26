# backlight-rs

## Installation
Make sure Rust is installed, see [rustup.sh](https://rustup.rs/)


- `git clone https://github.com/danielalvsaaker/backlight-rs`
- `cd backlight-rs`
- Change the path in src/main.rs depending on your system
- If you get an overflow error, find and replace `u16` with `u32` or `u64`

- `cargo build --release`  

Executable is located in target/release

## Running
`backlight increase <value>`  
`backlight decrease <value>`
