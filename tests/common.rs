use std::fs;

pub fn run_snapshot(name: &str) -> () {
  check_snapshot(read_snapshot(name));
}

fn check_snapshot((input, errfmt, expected): (String, String, String)) -> () {
  assert_eq!(expected, errfmt::run(input, errfmt).unwrap());
}

fn read_snapshot(name: &str) -> (String, String, String) {
  (
    read_partial_snapshot(name, "input"),
    read_partial_snapshot(name, "errfmt"),
    read_partial_snapshot(name, "expected"),
  )
}

fn read_partial_snapshot(name: &str, kind: &str) -> String {
  read_file(format!("./snapshots/{}/{}", name, kind))
}

fn read_file(name: String) -> String {
  fs::read_to_string(name)
    .unwrap()
    .trim_end_matches('\n')
    .to_string()
}
