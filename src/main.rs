mod args;

use std::process;
use clap::Parser;

use args::Args;
use syq::command_report;

fn main() {
  let args = Args::parse();

  let task = async {
    let result = command_report(&args.cookie, args.repeat).await;
    if let Err(e) = result {
      eprintln!("{}", e);
      process::exit(-1);
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
