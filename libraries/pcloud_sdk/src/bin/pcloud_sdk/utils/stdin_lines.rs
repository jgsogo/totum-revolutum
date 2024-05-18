use std::io;

use log::debug;

use crate::output::Porcelain;

/// Returns content line by line. Signature is equal to the one in [`std::io::read_line`]
///
/// Implements [`Iterator`] trait and parse lines taking into account output from
/// commands using `--porcelain`: comments are removed from every line, trailing blanks
/// are removed and empty lines are skipped
trait LineByLine {
    fn next_line(&mut self, buf: &mut String) -> io::Result<usize>;

    fn next_non_empty_line(&mut self) -> Option<String> {
        let mut buffer = String::new();
        while let Ok(u) = self.next_line(&mut buffer) {
            // If EOF, we are done reading stdin
            if u == 0 {
                debug!("Nothing left in stdin");
                return None;
            }
            let data = Porcelain::parse_line(&buffer);
            debug!("Got from stdin '{data}'");
            if !data.is_empty() {
                return Some(data);
            }
            buffer.clear();
        }
        None
    }
}

pub struct StdinLines;

/// Parse stdin returning lines one by one. It parses lines taking into account output
/// from commands using `--porcelain`.
impl LineByLine for StdinLines {
    fn next_line(&mut self, buf: &mut String) -> io::Result<usize> {
        io::stdin().read_line(buf)
    }
}

impl Iterator for StdinLines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_non_empty_line()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStdin<'a> {
        data: std::vec::IntoIter<&'a str>,
    }

    impl<'a> MockStdin<'a> {
        pub fn new(data: Vec<&'a str>) -> Self {
            Self { data: data.into_iter() }
        }
    }

    impl LineByLine for MockStdin<'_> {
        fn next_line(&mut self, buf: &mut String) -> io::Result<usize> {
            match self.data.next() {
                None => Ok(0),
                Some(d) => {
                    let size = d.len();
                    *buf = d.to_string();
                    Ok(size)
                }
            }
        }
    }

    #[test]
    fn test_parse_lines() {
        let mut stdin = MockStdin::new(vec!["a", "b"]);

        assert_eq!(stdin.next_non_empty_line(), Some("a".to_string()));
        assert_eq!(stdin.next_non_empty_line(), Some("b".to_string()));
        assert_eq!(stdin.next_non_empty_line(), None);
    }
}
