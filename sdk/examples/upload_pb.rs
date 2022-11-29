use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{env, thread};

use futures_util::FutureExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use pcloud_sdk::data::app_client_data::AppClientData;
use pcloud_sdk::methods::file::uploadfile::{PostUploadFile, UploadFileParams};
use pcloud_sdk::methods::file::uploadprogress::{UploadProgress, UploadProgressData};
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = AppClientData::read_from_file("secrets/app.json").unwrap();
    let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let pcloud = pcloud_sdk::client::HttpClient::authorize(app, addr).await?;

    let userinfo = pcloud.userinfo().await?;
    println!("{:#?}", userinfo);

    let listfolder_input = ListFolderInput::new_from_path(None);
    let listfolder = pcloud.listfolder(&listfolder_input).await?;
    //println!("{:#?}", listfolder);

    let folderid = listfolder.metadata.folderid.as_ref().unwrap();
    let working_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let files_to_upload = vec![
        format!("{}/examples/files/file20", working_dir),
        format!("{}/examples/files/file30", working_dir),
        format!("{}/examples/files/file21", working_dir),
        format!("{}/examples/files/file22", working_dir),
    ];

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let mut all_tasks = vec![];
    for file in files_to_upload {
        let mut s = DefaultHasher::new();
        file.to_string().hash(&mut s);
        let progresshash = s.finish().to_string();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        {
            // This is the task to upload the file
            let pcloud = pcloud.clone();
            let file = file.to_string();
            let progresshash = progresshash.clone();
            all_tasks.push(
                async move {
                    let mut upload_params =
                        UploadFileParams::new_from_folderid(folderid.clone(), file.to_string());
                    upload_params.progresshash = Some(progresshash);
                    tx.send(()).unwrap();
                    let _r = pcloud.uploadfile(&file, upload_params).await.unwrap();
                    //r.fileids
                }
                .boxed(),
            );
        }

        {
            // This is the task for the progress bar
            // TODO: Probably it would be a better approach to delegate CLI events on some thread
            //  and post to it (using general-purpose channel) the progresshash of the files
            //  that are being uploaded. For each file, it can add a progress-bar entry to track
            //  upload status.
            let pcloud = pcloud.clone();
            let m = m.clone();
            let file = file.to_string();
            let sty = sty.clone();
            all_tasks.push(
                async move {
                    rx.await.ok().unwrap();
                    // FIXME: I need to wait here so the upload actually starts and progresshash is available on server side
                    thread::sleep(Duration::from_millis(300));
                    let pb_points = 1000;
                    let pb = m.add(ProgressBar::new(pb_points));
                    pb.set_style(sty);
                    loop {
                        let xx = pcloud.uploadprogress(&progresshash).await;
                        match xx {
                            Ok(UploadProgressData {
                                total,
                                uploaded,
                                finished: false,
                                ..
                            }) => {
                                let percent =
                                    ((uploaded as f32 / total as f32) * pb_points as f32) as u64;
                                pb.set_message(format!("{} #{}", &file, percent));
                                pb.set_position(percent);
                            }
                            Ok(UploadProgressData { finished: true, .. }) => {
                                pb.set_position(pb_points);
                                pb.finish_with_message(format!("{} #done!", &file));
                                break;
                            }
                            Err(e) => {
                                // TODO: Propagate actual error
                                pb.set_position(pb_points);
                                pb.finish_with_message(format!("{} #error! {}", &file, e));
                                break;
                            }
                        }
                        thread::sleep(Duration::from_millis(15));
                    }
                }
                .boxed(),
            );
        }
    }

    let _result = futures::future::join_all(all_tasks).await;
    println!("Done");
    Ok(())
}
