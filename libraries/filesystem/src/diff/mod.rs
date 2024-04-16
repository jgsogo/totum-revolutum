pub use file_pair::FilePair;
pub use two_ways::run as two_ways_run;

mod file_pair;
mod two_ways;

// TODO: Move this module out of this crate, it doesn't make sense here
