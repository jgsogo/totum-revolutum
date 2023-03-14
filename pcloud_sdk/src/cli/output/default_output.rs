use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;

use pcloud_sdk::methods::folder::ListFolder;
use pcloud_sdk::methods::general::UserInfo;
use pcloud_sdk::progress_bar::{NoProgressBar, ProgressBar, ProgressBarBuilder};

use crate::output::Print;
use crate::CliParams;

pub struct DefaultOutput {
    sty: indicatif::ProgressStyle,
    pbs: indicatif::MultiProgress,

    use_progress_bars: bool,
}

impl DefaultOutput {
    pub fn new(cli_params: &CliParams) -> Self {
        // Configure progressbar and output
        let sty =
            indicatif::ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-");
        Self {
            sty,
            pbs: indicatif::MultiProgress::new(),
            // Progress bars are not used if log level is more verbose than INFO
            use_progress_bars: cli_params.verbose.log_level_filter() <= log::LevelFilter::Info,
        }
    }
}

impl ProgressBarBuilder for DefaultOutput {
    fn build(&self, total_size: u64) -> Box<dyn ProgressBar> {
        if self.use_progress_bars {
            let r = self.pbs.add(indicatif::ProgressBar::new(total_size));
            r.set_style(self.sty.clone());
            Box::new(r)
        } else {
            Box::<NoProgressBar>::default()
        }
    }
}

impl Print for DefaultOutput {
    fn println(&self, text: &str) -> Result<()> {
        // By default it prints to stderr: https://docs.rs/indicatif/latest/indicatif/struct.MultiProgress.html#method.new
        println!("{text}");
        Ok(())
    }

    fn eprintln(&self, text: &str) -> Result<()> {
        self.pbs.println(format!("{text}")).map_err(|e| anyhow!(e))
    }

    fn path(&self, path: Utf8PathBuf) -> Result<()> {
        println!("{path}");
        Ok(())
    }

    fn list_folder(&self, list_folder: &ListFolder) -> Result<()> {
        println!("{:?}", list_folder);
        Ok(())
    }

    fn user_info(&self, user_info: &UserInfo) -> Result<()> {
        println!("{:?}", user_info);
        Ok(())
    }
}
