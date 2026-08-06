// @generated automatically by Diesel CLI.

diesel::table! {
    oauth_accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        provider -> Text,
        provider_user_id -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamptz>,
        scope -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        refresh_token_hash -> Text,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Inet>,
        expires_at -> Timestamptz,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        hytale_id -> Text,
        username -> Text,
        display_name -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        email -> Nullable<Text>,
        email_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(oauth_accounts -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(oauth_accounts, sessions, users,);
