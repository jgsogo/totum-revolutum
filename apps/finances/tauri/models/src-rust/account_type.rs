pub enum AccountCategory {
    Other,
    Savings,
    Investment,
    Retirement,
}

impl From<AccountCategory> for crate::protos::finances_app_models::AccountCategory {
    fn from(val: AccountCategory) -> Self {
        match val {
            AccountCategory::Other => crate::protos::finances_app_models::AccountCategory::Other,
            AccountCategory::Savings => crate::protos::finances_app_models::AccountCategory::Savings,
            AccountCategory::Investment => crate::protos::finances_app_models::AccountCategory::Investment,
            AccountCategory::Retirement => crate::protos::finances_app_models::AccountCategory::Retirement,
        }
    }
}

const SAVINGS_UNIQUE_NAMES: &[&str] = &[finances_accounts::constants::accounttype::ASSETS_CURRENT_SAVINGS];
const INVESTMENT_UNIQUE_NAMES: &[&str] = &[
    finances_investments::constants::accounttype::ASSETS_CURRENT_INVESTMENT,
    finances_investments::constants::accounttype::ASSETS_NON_CURRENT_REAL_STATE,
];
const RETIREMENT_UNIQUE_NAMES: &[&str] = &[finances_investments::constants::accounttype::ASSETS_NON_CURRENT_RETIREMENT];

impl AccountCategory {
    pub fn savings_accounttypes() -> &'static [&'static str] {
        SAVINGS_UNIQUE_NAMES
    }

    pub fn investment_accounttypes() -> &'static [&'static str] {
        INVESTMENT_UNIQUE_NAMES
    }

    pub fn retirement_accounttypes() -> &'static [&'static str] {
        RETIREMENT_UNIQUE_NAMES
    }
}

#[derive(Clone)]
pub struct AccountType(pub(crate) crate::protos::finances_app_models::AccountType);

impl AccountType {
    pub fn new(pk: i64, name: String, breadcrumb: Option<Vec<String>>, category: AccountCategory) -> Self {
        let category: crate::protos::finances_app_models::AccountCategory = category.into();
        Self(crate::protos::finances_app_models::AccountType {
            pk,
            name,
            breadcrumb: breadcrumb.unwrap_or_default(),
            category: category.into(),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }
}
