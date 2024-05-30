use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{ParseError, ParseErrorKind};

const FOLDERID_PREFIX: &str = "folderid:";
pub type InnerType = u64;

/// Unique identifier of a folder in PCloud storage
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FolderID(InnerType);

impl FolderID {
    pub fn new(value: InnerType) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> InnerType {
        self.0
    }
}

impl Display for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "{FOLDERID_PREFIX}{value}")
    }
}

impl Debug for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "{FOLDERID_PREFIX}{value:?}")
    }
}

impl FromStr for FolderID {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_str = || -> Result<_, ParseErrorKind> {
            let s = s
                .strip_prefix(FOLDERID_PREFIX)
                .ok_or(ParseErrorKind::NoFolderIDPrefix)?;
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
    fn test_folderid() {
        let folderid = FolderID(23);
        assert_eq!(folderid.to_string(), "folderid:23");
        assert_eq!(format!("{folderid:?}"), "folderid:23");
    }

    #[test]
    fn test_parse_folderid() {
        assert_eq!(FolderID::from_str("folderid:123").unwrap(), FolderID(123));
        assert!(FolderID::from_str("folderid-123").is_err());
    }

    #[test]
    fn fail_fileid_prefix() {
        let r = "other:123".parse::<FolderID>();
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "Cannot parse from string 'other:123': no FolderID prefix, missing `folderid:`"
        );
    }

    #[test]
    fn fail_integer_parse() {
        let r = "folderid:123a23".parse::<FolderID>();
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "Cannot parse from string 'folderid:123a23': Not valid integer value"
        );
    }
}
