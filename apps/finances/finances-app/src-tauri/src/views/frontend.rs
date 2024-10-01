//! Defines the types that are exposed to the frontend

#[derive(serde::Serialize, Debug)]
pub struct Holder {
    // pub pk: i32,
    pub name: String,
}

#[derive(serde::Serialize, Debug)]
pub struct AccountType {
    // pub pk: i32,
    pub name: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Account {
    // pub pk: i32,
    pub name: String,
    pub holder: Holder,
    pub r#type: AccountType,
}
