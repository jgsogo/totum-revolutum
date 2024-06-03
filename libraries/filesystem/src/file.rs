use crate::Result;
use async_trait::async_trait;

#[async_trait]
/// An object providing access to an opened file on a [`crate::Filesystem`]
///
/// Files are automatically closed when they go out of scope. Errors detected on closing
/// are ignored by the implementation of Drop. Use the method `sync_all` if these errors
/// must be manually handled.
pub trait File {
    /// Reads all bytes from the byte stream.
    ///
    /// All bytes read from this stream will be appended to the specified buffer `buf`.
    /// This function will continuously call [`File::read`] to append more data to `buf` until
    /// [`File::read`] returns either `Ok(0)` or an error.
    ///
    /// If successful, this function will return the total number of bytes read.
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;

    /// Reads some bytes from the byte stream.
    ///
    /// Returns the number of bytes read from the start of the buffer.
    ///
    /// If the return value is `Ok(n)`, then it must be guaranteed that
    /// `0 <= n <= buf.len()`. A nonzero `n` value indicates that the buffer has been
    /// filled in with `n` bytes of data
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    async fn write_all(&mut self, buf: &[u8]) -> Result<()>;

    async fn sync_all(&mut self) -> Result<()>;
}
