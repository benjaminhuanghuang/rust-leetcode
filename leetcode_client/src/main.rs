use leetcode_client::{get_args, run};

fn main() {
  // If either get_args or run returns an Err, print it to STDERR.
  if let Err(e) = get_args().and_then(run) {
    eprintln!("Error {}", e);
    std::process::exit(1);
  }
}
