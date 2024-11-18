// @generated automatically by Diesel CLI.

diesel::table! {
    auth_group (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
    }
}

diesel::table! {
    auth_group_permissions (id) {
        id -> Int8,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_permission (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        content_type_id -> Int4,
        #[max_length = 100]
        codename -> Varchar,
    }
}

diesel::table! {
    auth_user (id) {
        id -> Int4,
        #[max_length = 128]
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        #[max_length = 150]
        username -> Varchar,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
    }
}

diesel::table! {
    auth_user_groups (id) {
        id -> Int8,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    auth_user_user_permissions (id) {
        id -> Int8,
        user_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        #[max_length = 200]
        object_repr -> Varchar,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

diesel::table! {
    django_content_type (id) {
        id -> Int4,
        #[max_length = 100]
        app_label -> Varchar,
        #[max_length = 100]
        model -> Varchar,
    }
}

diesel::table! {
    django_migrations (id) {
        id -> Int8,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        applied -> Timestamptz,
    }
}

diesel::table! {
    django_session (session_key) {
        #[max_length = 40]
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
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

diesel::table! {
    finances_investments_movementdividend (movement_ptr_id) {
        movement_ptr_id -> Int8,
        ex_dividend_date -> Date,
        unit_value -> Numeric,
    }
}

diesel::table! {
    finances_investments_movementnumerable (movement_ptr_id) {
        movement_ptr_id -> Int8,
        quantity -> Numeric,
        unit_value -> Numeric,
    }
}

diesel::table! {
    finances_investments_snapshotnumerable (id) {
        id -> Int8,
        date_value -> Date,
        quantity -> Numeric,
        unit_value -> Numeric,
        account_id -> Int8,
    }
}

diesel::joinable!(auth_group_permissions -> auth_group (group_id));
diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_permission -> django_content_type (content_type_id));
diesel::joinable!(auth_user_groups -> auth_group (group_id));
diesel::joinable!(auth_user_groups -> auth_user (user_id));
diesel::joinable!(auth_user_user_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_user_user_permissions -> auth_user (user_id));
diesel::joinable!(django_admin_log -> auth_user (user_id));
diesel::joinable!(django_admin_log -> django_content_type (content_type_id));
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
diesel::joinable!(finances_investments_movementdividend -> finances_accounts_movement (movement_ptr_id));
diesel::joinable!(finances_investments_movementnumerable -> finances_accounts_movement (movement_ptr_id));
diesel::joinable!(finances_investments_snapshotnumerable -> finances_accounts_account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_group,
    auth_group_permissions,
    auth_permission,
    auth_user,
    auth_user_groups,
    auth_user_user_permissions,
    django_admin_log,
    django_content_type,
    django_migrations,
    django_session,
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
    finances_investments_movementdividend,
    finances_investments_movementnumerable,
    finances_investments_snapshotnumerable,
);
