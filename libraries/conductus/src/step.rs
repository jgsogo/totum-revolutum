// use async_trait::async_trait;
//
// use crate::Result;
//
// /// Every pipeline step implements this trait
// #[async_trait]
// pub trait Step {
//     type Item;
//
//     async fn take(&mut self, n: usize) -> Result<Vec<Self::Item>>;
// }
