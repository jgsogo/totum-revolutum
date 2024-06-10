use crate::Result;
use async_trait::async_trait;

#[async_trait]
/// An object providing access to an opened file inside a [`crate::Filesystem`]
///
/// When the object is dropped, the underlying file will automatically be closed. However, it's up
/// to the specific implementations to define how [`Drop`] is implemented and/or close is
/// unconditionally guaranteed even if the application dies.
///
/// The method [`Self::sync_all`] is provided so the user can wait and receive any error that
/// might happen while closing the file.
pub trait File: Send {
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

    /// Write an entire buffer into the file
    async fn write_all(&mut self, buf: &[u8]) -> Result<()>;

    // /// This function ensures that all in-memory data reaches the filesystem. After executing this
    // /// method it can be assumed that `Drop` will close the file successfully (or without any
    // /// loss of information).
    // async fn sync_all(&mut self) -> Result<()>;
}
