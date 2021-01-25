use bzfquery;

const DEFAULT_PORT: u16 = 5154;

fn main() {
  let mut args: Vec<String> = std::env::args().collect();
  args.remove(0); // remove first arguement which is self

  if args.len() != 1 || args[0] == "-h" || args[0] == "--help" {
    println!("{} v{}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("    {} <address:port>", env!("CARGO_PKG_NAME"));
    std::process::exit(0);
  }

  let parts: Vec<&str> = args[0].split(":").collect();
  let host = parts[0];
  let port = match parts.len() {
    2 => parts[1].parse().unwrap_or(DEFAULT_PORT),
    _ => DEFAULT_PORT,
  };

  let query = bzfquery::query(host, port);
  println!("{}", query);
}
