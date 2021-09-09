/*
 * Copyright 2021. gudaoxuri
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct AccountGroupAddReq {
    // 关联账号Id
    #[validate(length(max = 32))]
    pub rel_account_id: String,
    // 关联群组节点Id
    #[validate(length(max = 32))]
    pub rel_group_node_id: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct AccountRoleAddReq {
    // 关联账号Id
    #[validate(length(max = 32))]
    pub rel_account_id: String,
    // 关联角色Id
    #[validate(length(max = 32))]
    pub rel_role_id: String,
}