extern crate clap;
use clap::{App, Arg};

use std::io;
use std::io::Read;

fn main() {
  let args = App::new("errfmt")
    .version("0.0.1")
    .author("mzanibelli")
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
  invoke_parser(
    args.value_of("errfmt").unwrap().to_string(),
    args.value_of("file").unwrap_or("").to_string(),
  )
  .map(|output| {
    if !String::is_empty(&output) {
      println!("{}", output)
    }
  })
  .unwrap_or_else(|err| eprintln!("{}", err.to_string()))
}

fn invoke_parser(errfmt: String, file: String) -> Result<String, String> {
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
