use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::anyhow;
use async_std::fs::File as AsyncFile;
use async_trait::async_trait;
use futures::io::AsyncReadExt;

use crate::diff::File;

pub struct LocalFile {
    file: AsyncFile,
}

impl LocalFile {
    pub fn new(file: async_std::fs::File) -> Self {
        Self { file }
    }
}

#[async_trait]
impl File for LocalFile {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> anyhow::Result<usize> {
        self.file.read_to_end(buf).await.map_err(|e| anyhow!(e))
    }
}

// Implementing this trait is required to implement `async_std::io::Write`. See: https://docs.rs/async-std/0.99.4/async_std/io/trait.Write.html
impl futures::io::AsyncWrite for LocalFile {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        unsafe {
            let a = Pin::get_unchecked_mut(self);
            let boxed = Pin::new(&mut a.file);
            boxed.poll_write(cx, buf)
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        unsafe {
            let a = Pin::get_unchecked_mut(self);
            let boxed = Pin::new(&mut a.file);
            boxed.poll_flush(cx)
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        unsafe {
            let a = Pin::get_unchecked_mut(self);
            let boxed = Pin::new(&mut a.file);
            boxed.poll_close(cx)
        }
    }
}
