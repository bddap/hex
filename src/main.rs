extern crate clap;
use {clap::{App, SubCommand},
     std::{io, io::Read, io::Write, process}};

fn main() {
    let matches = App::new("hex pipe converter")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("to").about("Print stdin as hex."))
        .subcommand(SubCommand::with_name("from").about("Print hex from stdin as raw."))
        .get_matches();

    match matches.subcommand() {
        ("to", _) => to().expect("IO error"),
        ("from", _) => from().expect("IO error"),
        _ => {
            eprintln!("Invalid sub command. Try '--help'.");
            process::exit(1);
        }
    }
}

fn to() -> io::Result<()> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (stdin, mut stdout) = (stdin.lock(), stdout.lock());
    for byte in stdin.bytes() {
        write!(&mut stdout, "{:02x}", byte?)?;
    }
    Ok(())
}

fn from() -> Result<(), ()> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (stdin, mut stdout) = (stdin.lock(), stdout.lock());
    let mut bytes = stdin.bytes();

    fn from_char(c: u8) -> Result<u8, ()> {
        match c {
            a @ b'0'...b'9' => Ok(a - b'0'),
            a @ b'a'...b'f' => Ok(a - b'a' + 10),
            _ => return Err(()),
        }
    }

    loop {
        match (bytes.next(), bytes.next()) {
            (Some(Ok(byte_a)), Some(Ok(byte_b))) => {
                let decoded: u8 = 16 *  from_char(byte_a)? + from_char(byte_b)?;
                let a = stdout.write_all(&[decoded]);
                if let Err(_) = a {
                    return Err(());
                }
            }
            (None, None) => return Ok(()), // EOF
            _ => return Err(()),
        }
    }
}
