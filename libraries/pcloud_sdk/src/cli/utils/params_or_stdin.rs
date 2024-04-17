use tracing::debug;

use super::stdin_lines::StdinLines;

pub enum ParamsOrStdin<T> {
    Params(std::vec::IntoIter<T>),
    Stdin(StdinLines),
}

impl<T> ParamsOrStdin<T> {
    pub fn new(params: Vec<T>) -> Self {
        if params.is_empty() {
            debug!("No files provided, will iterate from stdin");
            ParamsOrStdin::Stdin(StdinLines {})
        } else {
            ParamsOrStdin::Params(params.into_iter())
        }
    }
}

impl<T: From<String>> Iterator for ParamsOrStdin<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ParamsOrStdin::Params(p) => p.next(),
            ParamsOrStdin::Stdin(s) => s.next().map(|line| line.into()),
        }
    }
}
