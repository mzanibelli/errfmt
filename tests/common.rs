use std::fs;

pub fn run_snapshot(name: &str, errfmt: &str) -> () {
  check_snapshot(read_snapshot(name), errfmt.to_string());
}

fn check_snapshot((input, expected): (String, String), errfmt: String) -> () {
  assert_eq!(
    expected,
    errfmt::run(input, errfmt, String::new())
      .unwrap()
      .join("\n")
  );
}

fn read_snapshot(name: &str) -> (String, String) {
  (
    read_partial_snapshot(name, "input"),
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
