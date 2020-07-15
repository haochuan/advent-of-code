pub mod day01;
#[macro_use]
pub mod macros;

pub use self::error::Error;
pub use self::reader::Reader;

mod error {
    use std::fmt;
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Custom(String),
        Io(io::Error),
        ParseInt(std::num::ParseIntError),
    }

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Self::Io(e)
        }
    }

    impl From<std::num::ParseIntError> for Error {
        fn from(e: std::num::ParseIntError) -> Self {
            Self::ParseInt(e)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
                Self::Io(s) => write!(f, "{}", s),
                Self::ParseInt(s) => write!(f, "{}", s),
            }
        }
    }

    impl std::error::Error for Error {}
}

mod reader {
    use std::fs;
    use std::io;

    pub enum Reader<'a> {
        File(io::BufReader<fs::File>),
        Stdin(io::StdinLock<'a>),
    }

    impl<'a> io::Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self {
                Self::File(file) => file.read(buf),
                Self::Stdin(guard) => guard.read(buf),
            }
        }
    }

    impl<'a> io::BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            match self {
                Self::File(reader) => reader.fill_buf(),
                Self::Stdin(guard) => guard.fill_buf(),
            }
        }

        fn consume(&mut self, amt: usize) {
            match self {
                Self::File(reader) => reader.consume(amt),
                Self::Stdin(guard) => guard.consume(amt),
            }
        }
    }
}
