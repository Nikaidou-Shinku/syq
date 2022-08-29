use clap::Parser;

/// A simple program for the fucking silly epidemic reporting system.
#[derive(Parser)]
#[clap(author = "yurzhang", version, about, long_about = None)]
pub struct Args {
  /// Your username
  #[clap(short, long)]
  pub username: Option<String>,

  /// Your password
  #[clap(short, long)]
  pub password: Option<String>,

  /// Keep reporting until successful
  #[clap(short, long)]
  pub repeat: bool,
}
