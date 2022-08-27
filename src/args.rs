use clap::Parser;

/// Simple program for the fucking silly epidemic reporting system.
#[derive(Parser)]
#[clap(author = "yurzhang", version, about, long_about = None)]
pub struct Args {
  /// The cookie `JSESSIONID`
  #[clap(short, long)]
  pub cookie: String,

  /// Keep reporting until successful
  #[clap(short, long)]
  pub repeat: bool,
}
