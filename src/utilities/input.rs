use std::{str::FromStr, io, fmt, fmt::Display};

pub enum ReadError<T: FromStr> {
    IOError,
    ParsingError(T::Err),
}

impl<T> Display for ReadError<T>
where T: FromStr,
      T::Err: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError => write!(f, "Unknown IO error."),
            Self::ParsingError(error) => write!(f, "{error}"),
        }
    }
}

pub fn read_safe<T>(message: &str) -> T
where T: FromStr,
      T::Err: Display
{
    loop {
        match read(message) {
            Ok(value) => { return value },
            Err(error) => { println!("ERROR: {error}. Please try again.") },
        };
    }
}

pub fn read_while<T, P>(message: &str, predicate: P) -> T
where T: FromStr,
      T::Err: Display,
      P: Fn(&T) -> bool,
{
    loop {
        match read(message) {
            Ok(value) => { 
                if predicate(&value) {
                    continue; 
                } else {
                    return value;
                } 
            },
            Err(error) => { println!("ERROR: {error}. Please try again.") },
        }
    }
}

pub fn read<T: FromStr>(message: &str) -> Result<T, ReadError<T>> {
    println!("{message}");

    let mut buf = String::new();

    if io::stdin().read_line(&mut buf).is_err() {
        return Err(ReadError::IOError);
    }

    match buf.trim().parse::<T>() {
        Ok(value) => Ok(value),
        Err(error) => Err(ReadError::ParsingError(error)),
    }
}
