use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;

const FOLDERID_PREFIX: &str = "folderid";

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FolderID(pub u64);

impl Display for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "{FOLDERID_PREFIX}:{value}")
    }
}

impl Debug for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "{FOLDERID_PREFIX}:{value:?}")
    }
}

impl FromStr for FolderID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s
            .strip_prefix(FOLDERID_PREFIX)
            .and_then(|s| s.strip_prefix(':'))
            .ok_or(Error::ParseFolderIDError { string: s.to_string() })?;

        let x_fromstr = r
            .parse::<u64>()
            .map_err(|_| Error::ParseFolderIDError { string: s.to_string() })?;
        Ok(Self(x_fromstr))
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

        let r = FolderID::from_str("folderid-123");
        assert!(r.is_err());
    }
}
