extern crate clap;
use clap::{App, Arg};

use std::io;
use std::io::Read;

fn main() {
  let (errfmt, file) = parse_args();
  invoke_errfmt(errfmt, file)
    .map(|output| output.join("\n"))
    .map(|output| {
      if !String::is_empty(&output) {
        println!("{}", output)
      }
    })
    .unwrap_or_else(|err| eprintln!("{}", err))
}

fn parse_args() -> (String, String) {
  let args = App::new("errfmt")
    .about("Error messages formatter for kak(1)'s lint.kak script")
    .arg(
      Arg::with_name("errfmt")
        .short("e")
        .long("errfmt")
        .value_name("ERRFMT")
        .help("Vim-like errorformat string"),
    )
    .arg(
      Arg::with_name("file")
        .short("f")
        .long("file")
        .value_name("FILE")
        .help("The name that will replace every filepaths in the output"),
    )
    .get_matches();

  (
    args
      .value_of("errfmt")
      .unwrap_or(errfmt::PASSTHROUGH_ERRFMT)
      .to_string(),
    args.value_of("file").unwrap_or("").to_string(),
  )
}

fn invoke_errfmt(errfmt: String, file: String) -> Result<Vec<String>, String> {
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
