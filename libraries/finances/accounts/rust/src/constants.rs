// This file is auto-generated. Do not modify

// We use 'unique_name' fields to identify these elements because the PK might be different depending on the status of the DB when the data is migrated

pub mod accounttype {
    pub const ASSETS: &str = "/assets";
    pub const LIABILITIES: &str = "/liabilities";
    pub const ASSETS_CURRENT: &str = "/assets/current";
    pub const ASSETS_NON_CURRENT: &str = "/assets/non-current";
    pub const ASSETS_CURRENT_CASH: &str = "/assets/current/cash";
    pub const ASSETS_CURRENT_BANK_ACCOUNT: &str = "/assets/current/bank-account";
    pub const ASSETS_CURRENT_CASH_FLOW: &str = "/assets/current/cash-flow";
}

pub mod movementtype {
    pub const EXPENSE: &str = "/expense";
    pub const INCOME: &str = "/income";
    pub const OPERATIONS: &str = "/operations";
    pub const TAXES: &str = "/taxes";
    pub const TAXES_SPECIAL_CONTRIBUTIONS: &str = "/taxes/special-contributions";
    pub const TAXES_INCOME_AND_VALUE: &str = "/taxes/income-and-value";
    pub const TAXES_FEES_AND_TOLLS: &str = "/taxes/fees-and-tolls";
    pub const TAXES_INCOME_AND_VALUE_DIRECT: &str = "/taxes/income-and-value/direct";
    pub const TAXES_INCOME_AND_VALUE_INDIRECT: &str = "/taxes/income-and-value/indirect";
}
