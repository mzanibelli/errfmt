#[macro_use]
extern crate clap;

use clap::App;
use std::io;
use std::io::Read;

fn main() {
  invoke_errfmt(parse_args())
    .map(|output| output.join("\n"))
    .map(|output| {
      if !String::is_empty(&output) {
        println!("{}", output)
      }
    })
    .unwrap_or_else(|err| eprintln!("{}", err))
}

fn parse_args() -> (String, String) {
  let config = load_yaml!("../cli.yml");
  let args = App::from_yaml(config).get_matches();
  (
    args
      .value_of("errfmt")
      .unwrap_or(errfmt::PASSTHROUGH_ERRFMT)
      .to_string(),
    args.value_of("file").unwrap_or("").to_string(),
  )
}

fn invoke_errfmt((errfmt, file): (String, String)) -> Result<Vec<String>, String> {
  stdin_lines().and_then(move |lines| errfmt::run(lines, errfmt, file))
}

fn stdin_lines() -> Result<String, String> {
  let mut lines = String::new();
  io::stdin()
    .lock()
    .read_to_string(&mut lines)
    .map_err(|err| err.to_string())?;
  Ok(lines)
}
