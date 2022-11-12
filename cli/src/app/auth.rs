use clap::Args;

#[derive(Args, Debug)]
pub struct AuthParams {
    name: Option<String>,
}

pub fn handle(params: &AuthParams) {
    print!("Auth application: {:?}", params);
}
