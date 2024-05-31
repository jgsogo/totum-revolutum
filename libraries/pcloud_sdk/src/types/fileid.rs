use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{ParseError, ParseErrorKind};

const FILEID_PREFIX: &str = "fileid:";
pub type InnerType = u64;

/// Unique identifier of a file in PCloud storage
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FileID(InnerType);

impl FileID {
    pub fn new(value: InnerType) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> InnerType {
        self.0
    }
}

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FileID(value) = self;
        write!(f, "{FILEID_PREFIX}{value}")
    }
}

impl Debug for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FileID(value) = self;
        write!(f, "{FILEID_PREFIX}{value:?}")
    }
}

impl FromStr for FileID {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_str = || -> Result<_, ParseErrorKind> {
            let s = s.strip_prefix(FILEID_PREFIX).ok_or(ParseErrorKind::NoFileIDPrefix)?;
            let parsed = InnerType::from_str(s).map_err(ParseErrorKind::ParseInt)?;
            Ok(Self(parsed))
        };

        from_str().map_err(|source| ParseError {
            string: s.to_string(),
            source,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fileid() {
        let fileid = FileID::new(23);
        assert_eq!(fileid.to_string(), "fileid:23");
        assert_eq!(format!("{fileid:?}"), "fileid:23");
    }

    #[test]
    fn test_parse_fileid() {
        assert_eq!(FileID::from_str("fileid:123").unwrap(), FileID::new(123));
        assert!(FileID::from_str("fileid-123").is_err());
    }

    #[test]
    fn fail_fileid_prefix() {
        let r = "other:123".parse::<FileID>();
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "Cannot parse from string 'other:123': no FileID prefix, missing `fileid:`"
        );
    }

    #[test]
    fn fail_integer_parse() {
        let r = "fileid:123a23".parse::<FileID>();
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "Cannot parse from string 'fileid:123a23': Not valid integer value"
        );
    }
}
