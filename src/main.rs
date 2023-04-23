use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use clap::Parser;
use std::path::PathBuf;

const BUF_SIZE: usize = 8192; // 8KB

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    input: Option<PathBuf>,
}

fn scan<R: Read, W: Write>(mut reader: R, mut writer: W, buffer: &mut [u8]) -> io::Result<()> {
    fn replace_byte(haystack: &mut [u8], needle: u8, replace: u8) {
        for i in haystack {
            if *i == needle {
                *i = replace;
            }
        }
    }

    while let Ok(byte_size) = reader.read(buffer) {
        if byte_size == 0 {
            break;
        }

        replace_byte(&mut buffer[0..byte_size], b';', b':');

        writer.write_all(&buffer[0..byte_size])?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let mut buf: [u8; BUF_SIZE] = [0u8; BUF_SIZE]; // 8k buffer, aligned on memory page size

    let writer = std::io::stdout().lock();

    match args.input {
        Some(file) => {
            let reader = BufReader::new(File::open(file)?);
            scan(reader, writer, &mut buf)
        }
        None => {
            let reader = std::io::stdin().lock();
            scan(reader, writer, &mut buf)
        }
    }
}
