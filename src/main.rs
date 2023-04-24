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
    fn replace_byte<const NEEDLE: u8>(haystack: &mut [u8], replace: u8) {
        // generic implementation capable of roughly 1.3 GB/s on my machine
        // for i in haystack.iter_mut() {
        //     if *i == NEEDLE {
        //         *i = replace;
        //     }
        // }

        // Now let's try some home-grown SIMD-goodness. This is potentially more fragile, relying on the
        // hardware running the application to have 8-byte registers. On my machine, this sped the
        // execution up to approx. 1.6 GB/s
        let needle_arr: u64 = u64::from_ne_bytes([
            NEEDLE, NEEDLE, NEEDLE, NEEDLE, NEEDLE, NEEDLE, NEEDLE, NEEDLE,
        ]);

        let mut i = 0usize;

        let mut sub_haystack: [u8; 8];

        while i + 8 < haystack.len() {
            // Safe to unwrap because of the check in the while loop
            sub_haystack = haystack[i..i + 8].try_into().unwrap();
            let sub_haystack = u64::from_ne_bytes(sub_haystack);

            if sub_haystack & needle_arr > 0 {
                // There may be a smarter way to use the information we obtain from
                // sub_haystack & needle_arr, but I can't think of anything off the top of my head
                // that isn't just looping anyway
                for i in haystack[i..i + 8].iter_mut() {
                    if *i == NEEDLE {
                        *i = replace;
                    }
                }
            }
            i += 8;
        }

        for i in haystack[i..].iter_mut() {
            if *i == NEEDLE {
                *i = replace;
            }
        }
    }

    while let Ok(byte_size) = reader.read(buffer) {
        if byte_size == 0 {
            break;
        }

        replace_byte::<b';'>(&mut buffer[0..byte_size], b':');

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
