use serde::{Deserialize, Serialize};
use tardis::chrono::{DateTime, Utc};
use tardis::db::sea_orm::*;
use tardis::web::poem_openapi::Object;

#[derive(Object, Serialize, Deserialize, Debug)]
pub struct RbumRelAttrAddReq {
    pub is_from: bool,
    #[oai(validator(min_length = "2", max_length = "2000"))]
    pub value: String,
}

#[derive(Object, Serialize, Deserialize, Debug)]
pub struct RbumRelAttrModifyReq {
    #[oai(validator(min_length = "2", max_length = "2000"))]
    pub value: Option<String>,
}

#[derive(Object, FromQueryResult, Serialize, Deserialize, Debug)]
pub struct RbumRelAttrDetailResp {
    pub id: String,
    pub is_from: bool,
    pub value: String,
    pub rel_rbum_kind_attr_id: String,
    pub rel_rbum_kind_attr_name: String,
    pub rel_rbum_rel_name: String,

    pub rel_app_id: String,
    pub rel_app_name: String,
    pub rel_tenant_id: String,
    pub rel_tenant_name: String,
    pub updater_id: String,
    pub updater_name: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}
