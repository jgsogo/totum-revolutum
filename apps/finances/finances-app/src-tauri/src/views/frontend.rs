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
    pub ccy: String,
    pub identifier: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct Snapshot {
    pub amount: f32,
    pub quantity: Option<i32>,
    pub unit_value: Option<f32>,
    pub date_value: String,
}
