use clap::Parser;
#[derive(Parser, Debug)]
#[command(
    name = "calculate sm3",
    author,                                   // 自动读取 Cargo.toml 的 authors
    version = env!("CARGO_PKG_VERSION"),
    long_version = concat!(
        "[",
        env!("CARGO_PKG_VERSION"),
        "] by ",
        env!("CARGO_PKG_AUTHORS")
    ),
)]
pub struct Args {
    /// file path
    #[arg(short, long)]
    pub file: String,
}
