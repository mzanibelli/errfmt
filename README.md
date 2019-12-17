# Errfmt

This program parses error messages from various compilers and
linters and outputs them in a format that is compatible with
[Kakoune](https://github.com/mawww/kakoune)'s format.

## Usage

Let's format a message outputed by PHP syntax checking tool:

```
php -l myfile.php | errfmt -e '%k: %m in %f on line %l'
```

As you can see, messages shape is configured via the `--errfmt` (`-e`)
flag. The syntax is heavily inspired from Vim's similar feature. See
Rust crate documentation for more details on supported placeholders.

Additionally, the `--file` flag can be used to specify a static filename
and override any mistake made by the linter (ie. when input comes
from STDIN).

## Installation

- Test: `make test`
- Build: `make`
- Install: `make install` (this uses `cargo install` under the hood)
- Open Rust documentation in browser: `make doc`

## Snapshots

Integration tests are added to the `snapshots` folder. Check this folder
for a quick glance at currently supported linters. Do not forget to add
tests in case you want to implement support for a new input format.
