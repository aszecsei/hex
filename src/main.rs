use byteorder::{ByteOrder, NativeEndian};
use human_panic::setup_panic;
use itertools::Itertools;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, SeekFrom},
};
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "hex", about = "A hexdump utility.", author, group = ArgGroup::with_name("format").required(false).multiple(true))]
struct Options {
    /// Enable one-byte octal display.
    #[structopt(short = "b", long = "one-byte-octal", group = "format")]
    one_byte_octal: bool,
    /// Enable one-byte character display.
    #[structopt(short = "c", long = "one-byte-char", group = "format")]
    one_byte_char: bool,
    /// Enable two-byte octal display.
    #[structopt(short = "o", long = "two-bytes-octal", group = "format")]
    two_bytes_octal: bool,
    /// Enable two-byte hexadecimal display.
    #[structopt(short = "x", long = "two-bytes-hex", group = "format")]
    two_bytes_hex: bool,
    /// Enable canonical hex+ASCII display.
    #[structopt(short = "C", long = "canonical", group = "format")]
    canonical: bool,
    /// Enable two-byte decimal display.
    #[structopt(short = "d", long = "two-bytes-decimal", group = "format")]
    decimal: bool,

    /// Interpret only `length` bytes of input.
    #[structopt(short = "n", long = "length")]
    length: Option<u64>,
    /// Skip `offset` bytes from the beginning of the input.
    #[structopt(short = "s", long = "skip")]
    skip: Option<u64>,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> io::Result<()> {
    setup_panic!();

    let mut opt = Options::from_args();
    if !opt.one_byte_octal
        && !opt.one_byte_char
        && !opt.two_bytes_octal
        && !opt.two_bytes_hex
        && !opt.canonical
        && !opt.decimal
    {
        opt.canonical = true;
    }

    let file = fs::File::open(&opt.input)?;
    let mut br = io::BufReader::new(file);

    let offset = if let Some(skip) = opt.skip {
        br.seek(SeekFrom::Start(skip))?;
        skip
    } else {
        0
    };

    let print_lines = |reader: &mut dyn io::Read| -> io::Result<()> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes)?;

        for (chunk_idx, chunk) in bytes.iter().chunks(16).into_iter().enumerate() {
            let print_idx = || {
                let idx = offset as usize + (chunk_idx * 16);
                print!("{:#010x}\t", idx);
            };

            let chunk_vec = chunk.collect_vec();

            if opt.one_byte_octal {
                print_idx();
                for byte in chunk_vec.iter() {
                    print!("{:03o}", byte);
                    print!(" ");
                }
                println!();
            }

            if opt.one_byte_char {
                print_idx();
                for &&byte in chunk_vec.iter() {
                    let escaped = match byte as char {
                        '\t' => "\\t".to_string(),
                        '\n' => "\\n".to_string(),
                        '\r' => "\\r".to_string(),
                        '\0' => "\\0".to_string(),
                        ch if (0x20..=0x7e).contains(&byte) => format!("{}", ch),
                        _ => format!("{:o}", byte),
                    };
                    print!("{:>03} ", escaped);
                }
                println!();
            }

            if opt.canonical {
                print_idx();

                let size = if chunk_vec.len() > 8 {
                    let (first, second) = chunk_vec.split_at(8);
                    for byte in first {
                        print!("{:02X}", byte);
                        print!(" ");
                    }
                    print!(" ");
                    for byte in second {
                        print!("{:02X}", byte);
                        print!(" ");
                    }
                    chunk_vec.len() * 3 + 1
                } else {
                    for byte in chunk_vec.iter() {
                        print!("{:02X}", byte);
                        print!(" ");
                    }
                    chunk_vec.len() * 3
                };
                if chunk_vec.len() < 16 {
                    let full = 16 * 3 + 1;
                    let remain = full - size;
                    for _i in 0..remain {
                        print!(" ");
                    }
                }

                print!("\t|");
                for &&byte in chunk_vec.iter() {
                    if byte.is_ascii() {
                        let ch = byte as char;
                        if ch.is_control() {
                            print!(".");
                        } else {
                            print!("{}", ch);
                        }
                    } else {
                        print!(".");
                    }
                }
                println!("|");
            }

            if opt.decimal {
                print_idx();

                let merged = chunk_vec.iter().batching(|it| match it.next() {
                    None => None,
                    Some(x) => match it.next() {
                        None => Some(**x as u16),
                        Some(y) => Some(NativeEndian::read_u16(&[**x, **y])),
                    },
                });

                for halfword in merged {
                    print!("{:05} ", halfword);
                }
                println!();
            }

            if opt.two_bytes_octal {
                print_idx();

                let merged = chunk_vec.iter().batching(|it| match it.next() {
                    None => None,
                    Some(x) => match it.next() {
                        None => Some(**x as u16),
                        Some(y) => Some(NativeEndian::read_u16(&[**x, **y])),
                    },
                });

                for halfword in merged {
                    print!("{:06o} ", halfword);
                }
                println!();
            }

            if opt.two_bytes_hex {
                print_idx();

                let merged = chunk_vec.iter().batching(|it| match it.next() {
                    None => None,
                    Some(x) => match it.next() {
                        None => Some(**x as u16),
                        Some(y) => Some(NativeEndian::read_u16(&[**x, **y])),
                    },
                });

                for halfword in merged {
                    print!("{:04X} ", halfword);
                }
                println!();
            }
        }

        Ok(())
    };

    if let Some(len) = opt.length {
        print_lines(&mut br.take(len))?;
    } else {
        print_lines(&mut br)?;
    }

    Ok(())
}
