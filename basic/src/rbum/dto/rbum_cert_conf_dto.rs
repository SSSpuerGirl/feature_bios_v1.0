use serde::{Deserialize, Serialize};
use tardis::basic::field::TrimString;
use tardis::chrono::{DateTime, Utc};
#[cfg(feature = "default")]
use tardis::db::sea_orm;
#[cfg(feature = "default")]
use tardis::web::poem_openapi;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "default", derive(poem_openapi::Object))]
pub struct RbumCertConfAddReq {
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "255")))]
    pub code: TrimString,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "255")))]
    pub name: TrimString,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub note: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub ak_note: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub ak_rule: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub sk_note: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub sk_rule: Option<String>,
    pub ext: Option<String>,
    pub sk_need: Option<bool>,
    pub sk_dynamic: Option<bool>,
    pub sk_encrypted: Option<bool>,
    pub repeatable: Option<bool>,
    pub is_basic: Option<bool>,
    pub is_ak_repeatable: Option<bool>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub rest_by_kinds: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(minimum(value = "1", exclusive = "false"))))]
    pub expire_sec: Option<u32>,
    #[cfg_attr(feature = "default", oai(validator(minimum(value = "1", exclusive = "false"))))]
    pub sk_lock_cycle_sec: Option<u32>,
    pub sk_lock_err_times: Option<u8>,
    #[cfg_attr(feature = "default", oai(validator(minimum(value = "1", exclusive = "false"))))]
    pub sk_lock_duration_sec: Option<u32>,
    pub coexist_num: Option<u32>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub conn_uri: Option<String>,

    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "255")))]
    pub rel_rbum_domain_id: String,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "255")))]
    pub rel_rbum_item_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "default", derive(poem_openapi::Object))]
pub struct RbumCertConfModifyReq {
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "255")))]
    pub name: Option<TrimString>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub note: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub ak_note: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub ak_rule: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub sk_note: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub sk_rule: Option<String>,
    pub ext: Option<String>,
    pub sk_need: Option<bool>,
    pub sk_encrypted: Option<bool>,
    pub repeatable: Option<bool>,
    pub is_basic: Option<bool>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub rest_by_kinds: Option<String>,
    #[cfg_attr(feature = "default", oai(validator(minimum(value = "1", exclusive = "false"))))]
    pub expire_sec: Option<u32>,
    #[cfg_attr(feature = "default", oai(validator(minimum(value = "1", exclusive = "false"))))]
    pub sk_lock_cycle_sec: Option<u32>,
    pub sk_lock_err_times: Option<u8>,
    #[cfg_attr(feature = "default", oai(validator(minimum(value = "1", exclusive = "false"))))]
    pub sk_lock_duration_sec: Option<u32>,
    pub coexist_num: Option<u32>,
    #[cfg_attr(feature = "default", oai(validator(min_length = "2", max_length = "2000")))]
    pub conn_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "default", derive(poem_openapi::Object, sea_orm::FromQueryResult))]
pub struct RbumCertConfSummaryResp {
    pub id: String,
    pub code: String,
    pub name: String,
    pub ak_rule: String,
    pub sk_rule: String,
    pub ext: String,
    pub sk_need: bool,
    pub sk_dynamic: bool,
    pub sk_encrypted: bool,
    pub repeatable: bool,
    pub is_basic: bool,
    pub is_ak_repeatable: bool,
    pub rest_by_kinds: String,
    pub expire_sec: u32,
    pub sk_lock_cycle_sec: u32,
    pub sk_lock_err_times: u8,
    pub sk_lock_duration_sec: u32,
    pub coexist_num: u32,
    pub conn_uri: String,

    pub rel_rbum_domain_id: String,
    pub rel_rbum_item_id: String,

    pub own_paths: String,
    pub owner: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "default", derive(poem_openapi::Object, sea_orm::FromQueryResult))]
pub struct RbumCertConfDetailResp {
    pub id: String,
    pub code: String,
    pub name: String,
    pub note: String,
    pub ak_note: String,
    pub ak_rule: String,
    pub sk_note: String,
    pub sk_rule: String,
    pub ext: String,
    pub sk_need: bool,
    pub sk_dynamic: bool,
    pub sk_encrypted: bool,
    pub repeatable: bool,
    pub is_basic: bool,
    pub is_ak_repeatable: bool,
    pub rest_by_kinds: String,
    pub expire_sec: u32,
    pub sk_lock_cycle_sec: u32,
    pub sk_lock_err_times: u8,
    pub sk_lock_duration_sec: u32,
    pub coexist_num: u32,
    pub conn_uri: String,
    pub rel_rbum_domain_id: String,
    pub rel_rbum_domain_name: String,
    pub rel_rbum_item_id: String,
    pub rel_rbum_item_name: String,

    pub own_paths: String,
    pub owner: String,
    pub owner_name: Option<String>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "default", derive(poem_openapi::Object, sea_orm::FromQueryResult))]
pub struct RbumCertConfIdAndExtResp {
    pub id: String,
    pub ext: String,
}
