use anyhow::Result;
use indicatif;
use tempfile::tempdir;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use pcloud_sdk::data;
use pcloud_sdk::handy::HandyClient;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::general::getapiserver::GetAPIServer;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::methods::streaming::getfilelink::{FileLink, GetFileLink, GetFileLinkInput};
use pcloud_sdk::progress_bar::{ProgressBar, ProgressBarBuilder};

struct OutputExample {}

impl ProgressBarBuilder for OutputExample {
    fn new(&self, total_size: u64) -> Box<dyn ProgressBar> {
        Box::new(indicatif::ProgressBar::new(total_size))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = data::app_client_data::AppClientData::read_from_file("secrets/app.json").unwrap();
    let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let pcloud = pcloud_sdk::client::HttpClient::authorize(app, addr).await?;

    let userinfo = pcloud.userinfo().await?;
    println!("{:#?}", userinfo);

    let apiserver = pcloud.getapiserver().await?;
    println!("{:#?}", apiserver);

    let listfolder_input = ListFolderInput::new_from_path(None);
    let listfolder = pcloud.listfolder(&listfolder_input).await?;

    let tmp_dir = tempdir().unwrap();
    let output_example = OutputExample {};

    for m in listfolder.metadata.contents.unwrap().iter() {
        if !m.common.isfolder.unwrap() {
            println!("Found file: {}", m.common.name.as_ref().unwrap());

            let flink = GetFileLinkInput::new_from_fileid(m.fileid.as_ref().unwrap());
            let temp_path = tmp_dir.path().join(m.common.name.as_ref().unwrap());
            pcloud
                .getfilelink_and_download(&flink, &temp_path, &output_example)
                .await?;
        }
    }

    Ok(())
}
