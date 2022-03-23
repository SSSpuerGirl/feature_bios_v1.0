use serde::{Deserialize, Serialize};
use tardis::basic::field::TrimString;
use tardis::chrono::{DateTime, Utc};
use tardis::db::sea_orm::FromQueryResult;
use tardis::web::poem_openapi::Object;

#[derive(Object, Serialize, Deserialize, Debug)]
pub struct IamCaAccountAddReq {
    #[oai(validator(min_length = "2", max_length = "255"))]
    pub name: TrimString,
    #[oai(validator(min_length = "2", max_length = "1000"))]
    pub icon: Option<String>,

    pub disabled: Option<bool>,
}

#[derive(Object, Serialize, Deserialize, Debug)]
pub struct IamCaAccountModifyReq {
    #[oai(validator(min_length = "2", max_length = "255"))]
    pub name: Option<TrimString>,
    #[oai(validator(min_length = "2", max_length = "1000"))]
    pub icon: Option<String>,

    pub disabled: Option<bool>,
}

#[derive(Object, FromQueryResult, Serialize, Deserialize, Debug)]
pub struct IamCaAccountSummaryResp {
    pub id: String,
    pub name: String,
    pub icon: String,

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,

    pub disabled: bool,
}

#[derive(Object, FromQueryResult, Serialize, Deserialize, Debug)]
pub struct IamCaAccountDetailResp {
    pub id: String,
    pub name: String,
    pub icon: String,

    pub updater_id: String,
    pub updater_name: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,

    pub disabled: bool,
}