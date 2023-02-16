use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;

const FILEID_PREFIX: &str = "fileid";

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FileID(pub u64);

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FileID(value) = self;
        write!(f, "{FILEID_PREFIX}:{value}")
    }
}

impl Debug for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FileID(value) = self;
        write!(f, "{FILEID_PREFIX}:{value:?}")
    }
}

impl FromStr for FileID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s
            .strip_prefix(FILEID_PREFIX)
            .and_then(|s| s.strip_prefix(":"))
            .ok_or(Error::ParseFileIDError { string: s.to_string() })?;

        let x_fromstr = r
            .parse::<u64>()
            .map_err(|_| Error::ParseFileIDError { string: s.to_string() })?;
        Ok(Self(x_fromstr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fileid() {
        let fileid = FileID(23);
        assert_eq!(fileid.to_string(), "fileid:23");
        assert_eq!(format!("{fileid:?}"), "fileid:23");
    }

    #[test]
    fn test_parse_fileid() {
        assert_eq!(FileID::from_str("fileid:123").unwrap(), FileID(123));

        let r = FileID::from_str("fileid-123");
        assert!(r.is_err());
    }
}
