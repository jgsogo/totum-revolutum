use super::stdin_lines::StdinLines;
use tracing::debug;

// TODO: Probably more complex than needed...
pub enum ParamsOrStdin {
    Params(std::vec::IntoIter<String>),
    Stdin(StdinLines),
}

impl ParamsOrStdin {
    pub fn new(params: Vec<String>) -> Self {
        if params.is_empty() {
            debug!("No files provided, will iterate from stdin");
            ParamsOrStdin::Stdin(StdinLines {})
        } else {
            ParamsOrStdin::Params(params.into_iter())
        }
    }
}

impl<'a> Iterator for ParamsOrStdin {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ParamsOrStdin::Params(p) => p.next(),
            ParamsOrStdin::Stdin(s) => s.next(),
        }
    }
}
