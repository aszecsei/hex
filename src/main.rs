use hex::LineWriter;
use human_panic::setup_panic;
use itertools::Itertools;
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
    let mut br = io::BufReader::new(file);

    let offset = if let Some(skip) = opt.skip {
        br.seek(SeekFrom::Start(skip.try_into().unwrap()))?;
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

    let print_lines = |reader: &mut dyn io::Read| -> io::Result<()> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes)?;

        for (chunk_idx, chunk) in bytes.iter().chunks(16).into_iter().enumerate() {
            let chunk_vec = chunk.copied().collect_vec();
            let chunk_data = hex::ChunkData {
                offset: offset + (chunk_idx as u128 * 16),
                chunk: &chunk_vec,
            };

            let mut stdout = std::io::stdout();
            for writer in writers.iter() {
                writer.print_idx(&chunk_data, &mut stdout)?;
                writer.print_chunk(&chunk_data, &mut stdout)?;
            }
        }

        Ok(())
    };

    if let Some(len) = opt.length {
        print_lines(&mut br.take(len.try_into().unwrap()))?;
    } else {
        print_lines(&mut br)?;
    }

    Ok(())
}
