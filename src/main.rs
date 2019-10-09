use std::env;
use std::io;
use std::io::Read;

fn main() {
  invoke_parser(first_cli_argument())
    .map(|output| {
      if !String::is_empty(&output) {
        println!("{}", output)
      }
    })
    .unwrap_or_else(|err| eprintln!("{}", err.to_string()))
}

fn first_cli_argument() -> String {
  env::args()
    .into_iter()
    .skip(1)
    .next()
    .expect("usage: errfmt <pattern>")
}

fn invoke_parser(errfmt: String) -> Result<String, String> {
  stdin_lines().and_then(move |lines| errfmt::run(lines, errfmt))
}

fn stdin_lines() -> Result<String, String> {
  let mut lines = String::new();
  io::stdin()
    .lock()
    .read_to_string(&mut lines)
    .map_err(|err| err.to_string())?;
  Ok(lines)
}
