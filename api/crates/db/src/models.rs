use chrono::{DateTime, Utc};
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::schema::{oauth_accounts, sessions, users};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub hytale_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub hytale_id: &'a str,
    pub username: &'a str,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = oauth_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OAuthAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scope: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = oauth_accounts)]
pub struct NewOAuthAccount<'a> {
    pub user_id: Uuid,
    pub provider: &'a str,
    pub provider_user_id: &'a str,
    pub access_token: &'a str,
    pub refresh_token: Option<&'a str>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scope: Option<&'a str>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
    pub user_id: Uuid,
    pub refresh_token_hash: &'a str,
    pub user_agent: Option<&'a str>,
    pub ip_address: Option<IpNetwork>,
    pub expires_at: DateTime<Utc>,
}
