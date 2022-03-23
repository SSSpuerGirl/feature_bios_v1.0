use serde::{Deserialize, Serialize};
use tardis::basic::field::TrimString;

#[derive(Serialize, Deserialize, Debug)]
pub struct IamCtTenantModifyReq {
    pub name: Option<TrimString>,
    pub icon: Option<String>,
    pub sort: Option<i32>,

    pub contact_phone: Option<String>,

    pub scope_level: Option<i32>,
    pub disabled: Option<bool>,
}