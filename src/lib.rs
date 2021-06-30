mod byte_unit;

use byte_unit::parse_bytes;
use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "hex", about = "A hexdump utility.", author, group = ArgGroup::with_name("format").required(false).multiple(true))]
pub struct Options {
  /// Enable one-byte octal display.
  #[structopt(short = "b", long = "one-byte-octal", group = "format")]
  pub one_byte_octal: bool,
  /// Enable one-byte character display.
  #[structopt(short = "c", long = "one-byte-char", group = "format")]
  pub one_byte_char: bool,
  /// Enable two-byte octal display.
  #[structopt(short = "o", long = "two-bytes-octal", group = "format")]
  pub two_bytes_octal: bool,
  /// Enable two-byte hexadecimal display.
  #[structopt(short = "x", long = "two-bytes-hex", group = "format")]
  pub two_bytes_hex: bool,
  /// Enable canonical hex+ASCII display.
  #[structopt(short = "C", long = "canonical", group = "format")]
  pub canonical: bool,
  /// Enable two-byte decimal display.
  #[structopt(short = "d", long = "two-bytes-decimal", group = "format")]
  pub decimal: bool,

  /// Interpret only `length` bytes of input.
  #[structopt(short = "n", long = "length", parse(try_from_str = parse_bytes))]
  pub length: Option<u128>,
  /// Skip `offset` bytes from the beginning of the input.
  #[structopt(short = "s", long = "skip", parse(try_from_str = parse_bytes))]
  pub skip: Option<u128>,

  /// Input file
  #[structopt(parse(from_os_str))]
  pub input: PathBuf,
}
