use hex::LineWriter;
use human_panic::setup_panic;
use std::convert::TryInto;
use std::io::prelude::*;
use std::{
    fs,
    io::{self, SeekFrom},
};
use structopt::StructOpt;

fn main() -> io::Result<()> {
    setup_panic!();

    let mut opt = hex::Options::from_args();
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
    let mut reader = io::BufReader::new(file);

    let offset = if let Some(skip) = opt.skip {
        reader.seek(SeekFrom::Start(skip.try_into().unwrap()))?;
        skip
    } else {
        0
    };

    let writers = {
        let mut writers: Vec<Box<dyn LineWriter>> = vec![];
        if opt.one_byte_octal {
            writers.push(Box::new(hex::OneByteOctal));
        }
        if opt.one_byte_char {
            writers.push(Box::new(hex::OneByteChar));
        }
        if opt.canonical {
            writers.push(Box::new(hex::CanonicalWriter));
        }
        if opt.decimal {
            writers.push(Box::new(hex::DecimalWriter));
        }
        if opt.two_bytes_octal {
            writers.push(Box::new(hex::TwoBytesOctal));
        }
        if opt.two_bytes_hex {
            writers.push(Box::new(hex::TwoBytesHex));
        }
        writers
    };

    if let Some(len) = opt.length {
        hex::print_lines(&writers, offset, &mut reader.take(len.try_into().unwrap()))?;
    } else {
        hex::print_lines(&writers, offset, &mut reader)?;
    }

    Ok(())
}
