use std::collections::HashMap;

pub trait AddToParams: Send {
    fn add_to_params(&self, params: &mut HashMap<String, String>);
}

impl AddToParams for HashMap<String, String> {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        params.extend(self.clone())
    }
}
