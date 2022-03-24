use serde::{Deserialize, Serialize};
use tardis::basic::field::TrimString;
use tardis::web::poem_openapi::Object;

#[derive(Object, Serialize, Deserialize, Debug)]
pub struct IamCtHttpResAddReq {
    #[oai(validator(min_length = "2", max_length = "255"))]
    pub name: TrimString,
    #[oai(validator(min_length = "2", max_length = "2000"))]
    pub uri_path: TrimString,
    #[oai(validator(min_length = "2", max_length = "1000"))]
    pub icon: Option<String>,
    pub sort: Option<i32>,

    #[oai(validator(min_length = "2", max_length = "255"))]
    pub method: TrimString,

    pub disabled: Option<bool>,
}

#[derive(Object, Serialize, Deserialize, Debug)]
pub struct IamCtHttpResModifyReq {
    #[oai(validator(min_length = "2", max_length = "255"))]
    pub name: Option<TrimString>,
    #[oai(validator(min_length = "2", max_length = "2000"))]
    pub uri_path: Option<TrimString>,
    #[oai(validator(min_length = "2", max_length = "1000"))]
    pub icon: Option<String>,
    pub sort: Option<i32>,

    #[oai(validator(min_length = "2", max_length = "255"))]
    pub method: Option<TrimString>,

    pub disabled: Option<bool>,
}