use tokio::sync::oneshot::error::TryRecvError;
use tracing::debug;

use pcloud_sdk::methods::file::uploadprogress::{UploadProgress, UploadProgressData};
use pcloud_sdk::progress_bar::ProgressBar;

pub async fn progressbar_for_progresshash(
    pcloud: &impl UploadProgress, // HttpClient<OAuth2TokenImpl>,
    progresshash: String,
    mut rx: tokio::sync::oneshot::Receiver<()>,
    pb: Box<dyn ProgressBar>,
    pb_points: u64,
) -> bool {
    loop {
        match rx.try_recv() {
            Err(TryRecvError::Empty) => {
                // File is still being uploaded
                let xx = pcloud.uploadprogress(&progresshash).await;
                match xx {
                    Ok(UploadProgressData {
                        total,
                        uploaded,
                        finished: false,
                        ..
                    }) => {
                        let percent = ((uploaded as f32 / total as f32) * pb_points as f32) as u64;
                        pb.set_position(percent);
                    }
                    Ok(UploadProgressData { finished: true, .. }) => {
                        debug!("Finish from progresshash");
                        pb.set_position(pb_points);
                        pb.finish();
                        return true;
                    }
                    Err(_) => {
                        // Some error is expected here. The progresshash is not available immediately
                    }
                }
            }
            Err(TryRecvError::Closed) => {
                debug!("Channel closed before getting 100% progress");
                return false;
            }
            Ok(_) => {
                debug!("Finish from file upload!");
                pb.finish();
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::Arc;

    use async_trait::async_trait;

    use pcloud_sdk::error::PCloudError;
    use pcloud_sdk::Result;

    use super::*;

    struct ProgressData {
        pb_progress: u64,
        pb_finished: bool,

        up_progress: u64,
        up_total: u64,
    }

    impl ProgressData {
        fn new(total: u64) -> Self {
            Self {
                pb_progress: 0,
                pb_finished: false,
                up_progress: 0,
                up_total: total,
            }
        }
    }

    // Implements a mock progress bar
    struct MockProgressBar {
        data: Arc<RefCell<ProgressData>>,
    }

    impl MockProgressBar {
        fn new(data: Arc<RefCell<ProgressData>>) -> Self {
            MockProgressBar { data }
        }
    }

    impl ProgressBar for MockProgressBar {
        fn set_message(&self, _message: &str) {}

        fn set_position(&self, position: u64) {
            self.data.borrow_mut().pb_progress = position;
        }

        fn finish_with_message(&self, _message: &str) {}

        fn finish(&self) {
            self.data.borrow_mut().pb_finished = true;
        }
    }

    // Needed because RefCell is not Sync (and it is required for the async trait below)
    unsafe impl Send for MockProgressBar {}

    // Implements a mock upload
    struct MockUploadProgress {
        error: bool,
        data: Arc<RefCell<ProgressData>>,
    }

    impl MockUploadProgress {
        fn new(data: Arc<RefCell<ProgressData>>, error: bool) -> Self {
            MockUploadProgress { data, error }
        }
    }

    // Needed because RefCell is not Sync (and it is required for the async trait below)
    unsafe impl Sync for MockUploadProgress {}
    unsafe impl Send for MockUploadProgress {}

    #[async_trait]
    impl UploadProgress for MockUploadProgress {
        async fn uploadprogress(&self, _progresshash: &str) -> Result<UploadProgressData> {
            if self.error {
                return Err(PCloudError::from((9999u16, Some("Returning error as expected".to_string()))).into());
            }

            if self.data.borrow().up_total > self.data.borrow().up_progress {
                self.data.borrow_mut().up_progress += 1;
                Ok(UploadProgressData {
                    total: self.data.borrow().up_total,
                    uploaded: self.data.borrow().up_progress,
                    currentfile: None,
                    files: vec![],
                    finished: false,
                })
            } else {
                Ok(UploadProgressData {
                    total: self.data.borrow().up_total,
                    uploaded: self.data.borrow().up_progress,
                    currentfile: None,
                    files: vec![],
                    finished: true,
                })
            }
        }
    }

    #[tokio::test]
    async fn test_upload_finishes_and_pb_too() {
        // Test: upload goes one unit at a time until finish is reported. When finish is reported
        // the inner loop should return.

        let pb_points = 42;
        let data = Arc::new(RefCell::new(ProgressData::new(pb_points)));

        let mock_upload_progress = MockUploadProgress::new(data.clone(), false);
        let pb = Box::new(MockProgressBar::new(data.clone()));
        let (_tx, rx) = tokio::sync::oneshot::channel();
        let r = progressbar_for_progresshash(&mock_upload_progress, "hash".to_string(), rx, pb, pb_points).await;

        assert!(r);
        assert_eq!(data.borrow().pb_finished, true);
        assert_eq!(data.borrow().up_progress, pb_points);
        assert_eq!(data.borrow().pb_progress, pb_points);
    }

    #[tokio::test]
    async fn test_upload_finishes_but_pb_idle() {
        // Test: upload finishes and tx is sent, but the progress bar didn't get any valid value.

        let (tx, rx) = tokio::sync::oneshot::channel();
        let pb_points = 42;
        let data = Arc::new(RefCell::new(ProgressData::new(pb_points)));
        let pb = Box::new(MockProgressBar::new(data.clone()));
        let mock_upload_progress = MockUploadProgress::new(data.clone(), true);
        let r = tokio::spawn(async move {
            let r = progressbar_for_progresshash(&mock_upload_progress, "hash".to_string(), rx, pb, pb_points).await;
            r
        });

        // Sent upload finishes
        let _ = tx.send(());

        let pb_result = r.await.unwrap();
        assert_eq!(pb_result, true); // function reports finish (signal was received)
        assert_eq!(data.borrow().pb_finished, true);
        assert_eq!(data.borrow().pb_progress, 0); // ...but no progress was recorded
    }

    #[tokio::test]
    async fn test_upload_goes_away() {
        // Test: upload thread goes away, the tx signal is not sent, the rx detects channel is closed.

        let (tx, rx) = tokio::sync::oneshot::channel();
        let pb_points = 42;
        let data = Arc::new(RefCell::new(ProgressData::new(pb_points)));
        let pb = Box::new(MockProgressBar::new(data.clone()));
        let mock_upload_progress = MockUploadProgress::new(data.clone(), true);
        let r = tokio::spawn(async move {
            let r = progressbar_for_progresshash(&mock_upload_progress, "hash".to_string(), rx, pb, pb_points).await;
            r
        });

        // Upload thread goes away
        drop(tx);

        let pb_result = r.await.unwrap();
        assert_eq!(pb_result, false); // function reports `false` (signal was NOT received)
        assert_eq!(data.borrow().pb_finished, false);
        assert_eq!(data.borrow().pb_progress, 0); // ...but no progress was recorded
    }
}
