use std::process;
use clap::Parser;

use syq::{Args, Config, command_report};

fn main() {
  let args = Args::parse();
  let config = Config::from_file("config.toml").unwrap_or_else(|e| {
    eprintln!("Failed to read config file:\n{}", e);
    process::exit(-1);
  });

  let task = async {
    let result = command_report(&args, &config).await;
    if let Err(e) = result {
      eprintln!("Failed to report:\n{}", e);
      process::exit(-2);
    } else {
      println!("Report successful!");
    }
  };

  tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(task);
}
