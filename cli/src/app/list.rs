use clap::Args;

#[derive(Args, Debug)]
pub struct ListParams {
    name: Option<String>,
}

pub fn handle(params: &ListParams) {
    print!("List applications: {:?}", params);
}

