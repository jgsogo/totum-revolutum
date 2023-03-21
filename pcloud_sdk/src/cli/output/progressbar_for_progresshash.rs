use tokio::sync::oneshot::error::TryRecvError;
use tracing::debug;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::methods::file::uploadprogress::{UploadProgress, UploadProgressData};
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::progress_bar::ProgressBar;

pub async fn progressbar_for_progresshash(
    pcloud: HttpClient<OAuth2TokenImpl>,
    progresshash: String,
    mut rx: tokio::sync::oneshot::Receiver<()>,
    pb: Box<dyn ProgressBar>,
    pb_points: u64,
) {
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
                        break;
                    }
                    Err(_) => {
                        // Some error is expected here. The progresshash is not available inmediately
                    }
                }
            }
            Err(TryRecvError::Closed) => {
                debug!("Channel closed before getting 100% progress");
                break;
            }
            Ok(_) => {
                debug!("Finish from file upload!");
                pb.finish();
                break;
            }
        }
    }
}
