use std::io::{self, Read, Write};

const BUF_SIZE: usize = 64 * 1024;

fn scan<R>(mut input: R, buf: &mut [u8]) -> io::Result<()>
where
    R: Read,
{
    let mut writer = std::io::stdout().lock();

    while let Ok(size) = input.read(buf) {
        if size == 0 {
            break;
        }
        for i in 0..size {
            if buf[i] == b';' {
                buf[i] = b':';
            }
        }
        write(&mut writer, &buf, size)?;
    }
    Ok(())
}

fn write<W>(output: &mut W, buf: &[u8], bytes: usize) -> io::Result<()>
where
    W: Write,
{
    output.write_all(&buf)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    // let _args = Args::parse();
    let mut buf: [u8; BUF_SIZE] = [0u8; BUF_SIZE]; // 8k buffer, aligned on memory page size

    let input = std::io::stdin().lock();
    let output = std::io::stdout().lock();

    scan(input, &mut buf)
}
