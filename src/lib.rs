#![warn(clippy::all)]

mod byte_unit;

use byte_unit::parse_bytes;
use byteorder::{ByteOrder, NativeEndian};
use itertools::Itertools;
use mark_last::MarkLastIterator;
use std::io::{self, Write};
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

pub struct ChunkData<'a> {
    pub offset: u128,
    pub chunk: &'a [u8],
}
pub trait LineWriter {
    fn print_idx(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "{:#010x}\t", data.offset)
    }

    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()>;
}

pub struct CanonicalWriter;
impl LineWriter for CanonicalWriter {
    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        let size = if data.chunk.len() > 8 {
            let (first, second) = data.chunk.split_at(8);
            for byte in first {
                write!(w, "{:02X} ", byte)?;
            }
            write!(w, " ")?;
            for (last, byte) in second.iter().mark_last() {
                write!(w, "{:02X}", byte)?;
                if !last {
                    write!(w, " ")?;
                }
            }
            data.chunk.len() * 3
        } else {
            for (last, byte) in data.chunk.iter().mark_last() {
                write!(w, "{:02X}", byte)?;
                if !last {
                    write!(w, " ")?;
                }
            }
            data.chunk.len() * 3 - 1
        };
        if data.chunk.len() < 16 {
            let full = 16 * 3;
            let remain = full - size;
            for _i in 0..remain {
                write!(w, " ")?;
            }
        }

        write!(w, "\t|")?;
        for &byte in data.chunk {
            if byte.is_ascii() {
                let ch = byte as char;
                if ch.is_control() {
                    write!(w, ".")?;
                } else {
                    write!(w, "{}", ch)?;
                }
            } else {
                write!(w, ".")?;
            }
        }
        writeln!(w, "|")?;
        Ok(())
    }
}

pub struct OneByteOctal;
impl LineWriter for OneByteOctal {
    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        for (last, byte) in data.chunk.iter().mark_last() {
            write!(w, "{:03o}", byte)?;
            if !last {
                write!(w, " ")?;
            }
        }
        writeln!(w)?;
        Ok(())
    }
}

pub struct OneByteChar;
impl LineWriter for OneByteChar {
    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        for (last, byte) in data.chunk.iter().copied().mark_last() {
            let escaped = match byte as char {
                '\t' => "\\t".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\0' => "\\0".to_string(),
                ch if (0x20..=0x7e).contains(&byte) => ch.to_string(),
                _ => format!("{:o}", byte),
            };
            write!(w, "{:>03}", escaped)?;
            if !last {
                write!(w, " ")?;
            }
        }
        writeln!(w)?;
        Ok(())
    }
}

pub struct DecimalWriter;
impl LineWriter for DecimalWriter {
    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        let merged = data.chunk.iter().batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => Some(*x as u16),
                Some(y) => Some(NativeEndian::read_u16(&[*x, *y])),
            },
        });
        for (last, halfword) in merged.mark_last() {
            write!(w, "{:05}", halfword)?;
            if !last {
                write!(w, " ")?;
            }
        }
        writeln!(w)?;
        Ok(())
    }
}

pub struct TwoBytesOctal;
impl LineWriter for TwoBytesOctal {
    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        let merged = data.chunk.iter().batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => Some(*x as u16),
                Some(y) => Some(NativeEndian::read_u16(&[*x, *y])),
            },
        });
        for (last, halfword) in merged.mark_last() {
            write!(w, "{:06o}", halfword)?;
            if !last {
                write!(w, " ")?;
            }
        }
        writeln!(w)?;
        Ok(())
    }
}

pub struct TwoBytesHex;
impl LineWriter for TwoBytesHex {
    fn print_chunk(&self, data: &ChunkData<'_>, w: &mut dyn Write) -> io::Result<()> {
        let merged = data.chunk.iter().batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => Some(*x as u16),
                Some(y) => Some(NativeEndian::read_u16(&[*x, *y])),
            },
        });
        for (last, halfword) in merged.mark_last() {
            write!(w, "{:04X}", halfword)?;
            if !last {
                write!(w, " ")?;
            }
        }
        writeln!(w)?;
        Ok(())
    }
}

fn read_to_fill(reader: &mut dyn io::Read, buf: &mut [u8]) -> io::Result<usize> {
    let mut read = 0;
    loop {
        let buf = &mut buf[read..];
        match reader.read(buf) {
            Ok(0) => return Ok(read),
            Ok(n) => {
                assert!(n <= buf.len());
                read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
        if read == buf.len() {
            return Ok(read);
        }
    }
}

pub fn print_lines(
    writers: &[Box<dyn LineWriter>],
    offset: u128,
    reader: &mut dyn io::Read,
) -> io::Result<()> {
    const BUFFER_SIZE: usize = 16;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut idx = 0;
    loop {
        let amt = read_to_fill(reader, &mut buffer)?;
        let complete = amt < buffer.len();

        let chunk_data = ChunkData {
            offset: offset + (idx * BUFFER_SIZE as u128),
            chunk: &buffer[..amt],
        };

        let mut stdout = std::io::stdout();
        for writer in writers.iter() {
            writer.print_idx(&chunk_data, &mut stdout)?;
            writer.print_chunk(&chunk_data, &mut stdout)?;
        }

        idx += 1;

        if complete {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [u8; 16] = [
        0x54, 0x68, 0x69, 0x73, 0x01, 0x69, 0x73, 0x20, 0x61, 0x20, 0x63, 0x68, 0x75, 0x6E, 0x6B,
        0xFF,
    ];
    const TEST_CHUNK: ChunkData = ChunkData {
        offset: 16,
        chunk: &TEST_DATA,
    };

    #[test]
    fn canonical() {
        let mut out = vec![];
        let w = CanonicalWriter;
        w.print_idx(&TEST_CHUNK, &mut out).unwrap();
        w.print_chunk(&TEST_CHUNK, &mut out).unwrap();
        assert_eq!(
            out,
            b"0x00000010\t54 68 69 73 01 69 73 20  61 20 63 68 75 6E 6B FF\t|This.is a chunk.|\n"
        );
    }

    #[test]
    fn one_byte_octal() {
        let mut out = vec![];
        let w = OneByteOctal;
        w.print_idx(&TEST_CHUNK, &mut out).unwrap();
        w.print_chunk(&TEST_CHUNK, &mut out).unwrap();
        assert_eq!(
            out,
            b"0x00000010\t124 150 151 163 001 151 163 040 141 040 143 150 165 156 153 377\n"
        );
    }

    #[test]
    fn one_byte_char() {
        let mut out = vec![];
        let w = OneByteChar;
        w.print_idx(&TEST_CHUNK, &mut out).unwrap();
        w.print_chunk(&TEST_CHUNK, &mut out).unwrap();
        assert_eq!(
            out,
            b"0x00000010\t  T   h   i   s   1   i   s       a       c   h   u   n   k 377\n"
        );
    }

    #[test]
    fn decimal() {
        let mut out = vec![];
        let w = DecimalWriter;
        w.print_idx(&TEST_CHUNK, &mut out).unwrap();
        w.print_chunk(&TEST_CHUNK, &mut out).unwrap();
        assert_eq!(
            out,
            b"0x00000010\t26708 29545 26881 08307 08289 26723 28277 65387\n"
        );
    }

    #[test]
    fn two_bytes_octal() {
        let mut out = vec![];
        let w = TwoBytesOctal;
        w.print_idx(&TEST_CHUNK, &mut out).unwrap();
        w.print_chunk(&TEST_CHUNK, &mut out).unwrap();
        assert_eq!(
            out,
            b"0x00000010\t064124 071551 064401 020163 020141 064143 067165 177553\n"
        );
    }

    #[test]
    fn two_bytes_hex() {
        let mut out = vec![];
        let w = TwoBytesHex;
        w.print_idx(&TEST_CHUNK, &mut out).unwrap();
        w.print_chunk(&TEST_CHUNK, &mut out).unwrap();
        assert_eq!(
            out,
            b"0x00000010\t6854 7369 6901 2073 2061 6863 6E75 FF6B\n"
        );
    }
}
