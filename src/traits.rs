/// Any type that can construct itself from a buffered reader
pub trait FromReader: Sized {
    /// The kind of error that may occur during construction
    type Error: std::error::Error + Send + Sync + 'static;

    /// How the type constructs itself from a buffered reader
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error>;
}

impl FromReader for Vec<u8> {
    type Error = std::io::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl FromReader for String {
    type Error = std::io::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        Ok(string)
    }
}
