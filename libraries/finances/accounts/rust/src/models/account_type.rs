use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_accounttype)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(AccountType, foreign_key = tn_parent_id))]
pub struct AccountType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_abstract: bool,
    pub unique_name: Option<String>,

    // FIXME: These are treenode fields, implement them somwhere else if needed
    pub tn_parent_id: Option<i64>,
}

impl AccountType {
    /// Returns (a query to) all the [`AccountType`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_accounttype::table
    }

    /// Returns (a query to) all the [`AccountType`]s with 'unique_name
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_with_unique_name() -> _ {
        crate::schema::finances_accounts_accounttype::table
            .filter(crate::schema::finances_accounts_accounttype::unique_name.is_not_null())
    }

    /// Returns (a query to) all the [`AccountType`]s for a given unique_name
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn get_by_unique_name(unique_name: &str) -> _ {
        crate::schema::finances_accounts_accounttype::table
            .filter(crate::schema::finances_accounts_accounttype::unique_name.eq(unique_name))
    }

    // #[diesel::dsl::auto_type(no_type_alias)]
    // pub fn filter_branch<'a>(unique_name: &str) -> _ {
    //     // FIXME: Returns a query fragment to filter all the [`AccountType`] that are children of
    //     // the given [`AccountType`] (includes the given one as well)
    //     todo!("wip");
    // }

    // /// Returns a query fragment to filter all the [`Account`]s whose [`AccountType`] is either
    // /// [`CUENTA_CORRIENTE`] or [`METALICO`]
    // #[diesel::dsl::auto_type(no_type_alias)]
    // pub fn checking_account<'a>() -> _ {
    //     let cuenta_corriente: &'a str = CUENTA_CORRIENTE;
    //     let metalico: &'a str = METALICO;
    //     crate::schema::data_accounttype::name
    //         .eq(cuenta_corriente)
    //         .or(crate::schema::data_accounttype::name.eq(metalico))
    // }

    // /// Returns a query fragment to filter all the [`Account`]s whose [`AccountType`] is either
    // /// [`FONDO_INVERSION`], [`ACCIONES`], [`VIVIENDA`] or [`DEPOSITO`]
    // #[diesel::dsl::auto_type(no_type_alias)]
    // pub fn investment<'a>() -> _ {
    //     let fondo_inversion: &'a str = FONDO_INVERSION;
    //     let acciones: &'a str = ACCIONES;
    //     let vivienda: &'a str = VIVIENDA;
    //     let deposito: &'a str = DEPOSITO;

    //     crate::schema::data_accounttype::name
    //         .eq(fondo_inversion)
    //         .or(crate::schema::data_accounttype::name.eq(acciones))
    //         .or(crate::schema::data_accounttype::name.eq(vivienda))
    //         .or(crate::schema::data_accounttype::name.eq(deposito))
    // }

    // /// Returns a query fragment to filter all the [`Account`]s whose [`AccountType`] is
    // /// "Plan de pensiones"
    // #[diesel::dsl::auto_type(no_type_alias)]
    // pub fn retirement<'a>() -> _ {
    //     let plan_pensiones: &'a str = PLAN_PENSIONES;
    //     crate::schema::data_accounttype::name.eq(plan_pensiones)
    // }
}
