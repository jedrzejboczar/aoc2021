use std::{io::{BufRead, self, BufReader}, str::FromStr, error::Error, path::Path, fs::File};

/// Parse lines from input
///
/// Returns an iterator over parse results of non-empty lines in input.
pub fn parse_lines<I, T, E>(input: I) -> impl Iterator<Item = io::Result<T>>
    where
        I: BufRead,
        E: Into<Box<dyn Error + Send + Sync>>,
        T: FromStr<Err = E>,
{
    input.lines()
        .filter_map(|line| {
            // pass errors
            let line = match line {
                Ok(line) => line,
                Err(e) => return Some(Err(e))
            };
            // omit empty lines
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            // parse lines, comvert errors to io::Error
            let result = line.parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
            Some(result)
        })
}


/// Load and parse lines from file
///
/// Wrapper around `parse_lines` that reads from file
/// and collects the parsed results into a vector.
pub fn load_from_file<P, T, E>(path: P) -> io::Result<Vec<T>>
    where
        P: AsRef<Path>,
        E: Into<Box<dyn Error + Send + Sync>>,
        T: FromStr<Err = E>,
{
    let file = File::open(path)?;
    let input = BufReader::new(file);
    parse_lines(input)
        .collect()
}
