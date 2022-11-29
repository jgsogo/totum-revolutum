use anyhow::{Error, Result};
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

#[derive(Clone)]
struct OutputExample {
    sty: indicatif::ProgressStyle,
    pbs: indicatif::MultiProgress,
}

impl OutputExample {
    pub fn new(sty: indicatif::ProgressStyle) -> Self {
        Self {
            pbs: indicatif::MultiProgress::new(),
            sty,
        }
    }

    pub fn println(&self, msg: &str) {
        self.pbs.println(msg).expect("Failed to print");
    }
}

impl ProgressBarBuilder for OutputExample {
    fn new(&self, total_size: u64) -> Box<dyn ProgressBar> {
        let r = self.pbs.add(indicatif::ProgressBar::new(total_size));
        r.set_style(self.sty.clone());
        Box::new(r)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Configure progressbar and output
    let sty = indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    let output_example = OutputExample::new(sty);

    let app = data::app_client_data::AppClientData::read_from_file("secrets/app.json").unwrap();
    let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let pcloud = pcloud_sdk::client::HttpClient::authorize(app, addr).await?;

    let userinfo = pcloud.userinfo().await?;
    output_example.println(&format!("{:#?}", userinfo));

    let apiserver = pcloud.getapiserver().await?;
    output_example.println(&format!("{:#?}", apiserver));

    let listfolder_input = ListFolderInput::new_from_path(None);
    let listfolder = pcloud.listfolder(&listfolder_input).await?;

    let mut futures = tokio::task::JoinSet::new();
    let r = listfolder
        .metadata
        .contents
        .as_ref()
        .unwrap()
        .iter()
        .filter(|v| !v.common.isfolder.unwrap())
        .for_each(|v| {
            let metadata = v.clone();
            let pcloud = pcloud.clone();
            let output_example = output_example.clone();
            // let tmp_dir = tmp_dir.clone();
            futures.spawn(async move {
                let name = metadata.common.name.as_ref().unwrap();
                output_example.println(&format!("Found file: {}", name));
                let flink = GetFileLinkInput::new_from_fileid(metadata.fileid.as_ref().unwrap());
                let tmp_dir = tempdir().unwrap();
                let temp_path = tmp_dir.path().join(name);
                pcloud
                    .getfilelink_and_download(&flink, &temp_path, &output_example)
                    .await?;
                Ok::<String, Error>(name.to_string())
            });
        });

    while let Some(res) = futures.join_next().await {
        let _res = res.unwrap();
    }

    Ok(())
}
