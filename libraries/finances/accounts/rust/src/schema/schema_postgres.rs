// @generated automatically by Diesel CLI.

diesel::table! {
    django_migrations (id) {
        id -> Int4,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        applied -> Timestamptz,
    }
}

diesel::table! {
    finances_accounts_account (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        identifier -> Nullable<Varchar>,
        #[max_length = 3]
        ccy -> Varchar,
        open -> Date,
        close -> Nullable<Date>,
        type_id -> Int8,
        custodian_id -> Int8,
        is_numerable -> Bool,
    }
}

diesel::table! {
    finances_accounts_accountholder (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        is_company -> Bool,
    }
}

diesel::table! {
    finances_accounts_accountholderrole (id) {
        id -> Int8,
        owns_money -> Bool,
        account_id -> Int8,
        holder_id -> Int8,
    }
}

diesel::table! {
    finances_accounts_accounttype (id) {
        id -> Int8,
        tn_ancestors_pks -> Text,
        tn_ancestors_count -> Int4,
        tn_children_pks -> Text,
        tn_children_count -> Int4,
        tn_depth -> Int4,
        tn_descendants_pks -> Text,
        tn_descendants_count -> Int4,
        tn_index -> Int4,
        tn_level -> Int4,
        tn_priority -> Int4,
        tn_order -> Int4,
        tn_siblings_pks -> Text,
        tn_siblings_count -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_abstract -> Bool,
        tn_parent_id -> Nullable<Int8>,
        #[max_length = 255]
        unique_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    finances_accounts_custodian (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 2]
        country -> Varchar,
    }
}

diesel::table! {
    finances_accounts_fx (id) {
        id -> Int8,
        #[max_length = 3]
        foreign -> Varchar,
        #[max_length = 3]
        local -> Varchar,
        rate -> Numeric,
        date_value -> Date,
    }
}

diesel::table! {
    finances_accounts_movement (id) {
        id -> Int8,
        amount -> Numeric,
        direction -> Int4,
        date_value -> Date,
        account_id -> Int8,
        fx_id -> Nullable<Int8>,
        type_id -> Int8,
        transaction_id -> Int8,
    }
}

diesel::table! {
    finances_accounts_movementtype (id) {
        id -> Int8,
        tn_ancestors_pks -> Text,
        tn_ancestors_count -> Int4,
        tn_children_pks -> Text,
        tn_children_count -> Int4,
        tn_depth -> Int4,
        tn_descendants_pks -> Text,
        tn_descendants_count -> Int4,
        tn_index -> Int4,
        tn_level -> Int4,
        tn_priority -> Int4,
        tn_order -> Int4,
        tn_siblings_pks -> Text,
        tn_siblings_count -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_abstract -> Bool,
        tn_parent_id -> Nullable<Int8>,
        #[max_length = 255]
        unique_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    finances_accounts_snapshot (id) {
        id -> Int8,
        amount -> Numeric,
        date_value -> Date,
        account_id -> Int8,
    }
}

diesel::table! {
    finances_accounts_transaction (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        group_id -> Nullable<Int8>,
    }
}

diesel::table! {
    finances_accounts_transactiongroup (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 10]
        cadence -> Varchar,
        start -> Date,
    }
}

diesel::joinable!(finances_accounts_account -> finances_accounts_accounttype (type_id));
diesel::joinable!(finances_accounts_account -> finances_accounts_custodian (custodian_id));
diesel::joinable!(finances_accounts_accountholderrole -> finances_accounts_account (account_id));
diesel::joinable!(finances_accounts_accountholderrole -> finances_accounts_accountholder (holder_id));
diesel::joinable!(finances_accounts_movement -> finances_accounts_account (account_id));
diesel::joinable!(finances_accounts_movement -> finances_accounts_fx (fx_id));
diesel::joinable!(finances_accounts_movement -> finances_accounts_movementtype (type_id));
diesel::joinable!(finances_accounts_movement -> finances_accounts_transaction (transaction_id));
diesel::joinable!(finances_accounts_snapshot -> finances_accounts_account (account_id));
diesel::joinable!(finances_accounts_transaction -> finances_accounts_transactiongroup (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    django_migrations,
    finances_accounts_account,
    finances_accounts_accountholder,
    finances_accounts_accountholderrole,
    finances_accounts_accounttype,
    finances_accounts_custodian,
    finances_accounts_fx,
    finances_accounts_movement,
    finances_accounts_movementtype,
    finances_accounts_snapshot,
    finances_accounts_transaction,
    finances_accounts_transactiongroup,
);
