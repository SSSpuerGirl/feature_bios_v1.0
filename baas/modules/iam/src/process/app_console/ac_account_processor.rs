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

use actix_web::{delete, get, post, HttpRequest};
use sea_query::{Alias, Expr, JoinType, Order, Query};
use sqlx::Connection;
use strum::IntoEnumIterator;

use bios::basic::error::BIOSError;
use bios::db::reldb_client::SqlBuilderProcess;
use bios::web::basic_processor::get_ident_account_info;
use bios::web::resp_handler::{BIOSResp, BIOSRespHelper};
use bios::BIOSFuns;

use crate::domain::auth_domain::{IamAccountGroup, IamAccountRole, IamGroup, IamGroupNode, IamRole};
use crate::domain::ident_domain::IamAccount;
use crate::process::app_console::ac_account_dto::{AccountGroupDetailResp, AccountRoleDetailResp};
use crate::process::common::cache_processor;

#[post("/console/app/account/{account_id}/role/{role_id}")]
pub async fn add_account_role(req: HttpRequest) -> BIOSResp {
    let ident_info = get_ident_account_info(&req)?;
    let account_id: String = req.match_info().get("account_id").unwrap().parse()?;
    let role_id: String = req.match_info().get("role_id").unwrap().parse()?;
    let id = bios::basic::field::uuid();

    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccount::Id])
                .from(IamAccount::Table)
                .and_where(Expr::col(IamAccount::Id).eq(account_id.as_str()))
                .and_where(Expr::col(IamAccount::RelTenantId).eq(ident_info.tenant_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Account not exists".to_string()));
    }
    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamRole::Id])
                .from(IamRole::Table)
                .and_where(Expr::col(IamRole::Id).eq(role_id.as_str()))
                .and_where(Expr::col(IamRole::RelAppId).eq(ident_info.app_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Role not exists".to_string()));
    }

    if BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccountRole::Id])
                .from(IamAccountRole::Table)
                .and_where(Expr::col(IamAccountRole::RelRoleId).eq(role_id.as_str()))
                .and_where(Expr::col(IamAccountRole::RelAccountId).eq(account_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return Err(BIOSError::Conflict("IamAccountRole [rel_role_id] and [rel_account_id] already exists".to_string()));
    }

    BIOSFuns::reldb()
        .exec(
            &Query::insert()
                .into_table(IamAccountRole::Table)
                .columns(vec![
                    IamAccountRole::Id,
                    IamAccountRole::CreateUser,
                    IamAccountRole::UpdateUser,
                    IamAccountRole::RelAccountId,
                    IamAccountRole::RelRoleId,
                ])
                .values_panic(vec![
                    id.as_str().into(),
                    ident_info.account_id.as_str().into(),
                    ident_info.account_id.as_str().into(),
                    account_id.into(),
                    role_id.into(),
                ])
                .done(),
            None,
        )
        .await?;
    BIOSRespHelper::ok(id)
}

#[get("/console/app/account/{account_id}/role")]
pub async fn list_account_role(req: HttpRequest) -> BIOSResp {
    let ident_info = get_ident_account_info(&req)?;
    let account_id: String = req.match_info().get("account_id").unwrap().parse()?;

    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccount::Id])
                .from(IamAccount::Table)
                .and_where(Expr::col(IamAccount::Id).eq(account_id.as_str()))
                .and_where(Expr::col(IamAccount::RelTenantId).eq(ident_info.tenant_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Account not exists".to_string()));
    }

    let create_user_table = Alias::new("create");
    let update_user_table = Alias::new("update");
    let sql_builder = Query::select()
        .columns(vec![
            (IamAccountRole::Table, IamAccountRole::Id),
            (IamAccountRole::Table, IamAccountRole::CreateTime),
            (IamAccountRole::Table, IamAccountRole::UpdateTime),
            (IamAccountRole::Table, IamAccountRole::RelAccountId),
            (IamAccountRole::Table, IamAccountRole::RelRoleId),
        ])
        .expr_as(Expr::tbl(create_user_table.clone(), IamAccount::Name), Alias::new("create_user"))
        .expr_as(Expr::tbl(update_user_table.clone(), IamAccount::Name), Alias::new("update_user"))
        .from(IamAccountRole::Table)
        .join_as(
            JoinType::InnerJoin,
            IamAccount::Table,
            create_user_table.clone(),
            Expr::tbl(create_user_table, IamAccount::Id).equals(IamAccountRole::Table, IamAccountRole::CreateUser),
        )
        .join_as(
            JoinType::InnerJoin,
            IamAccount::Table,
            update_user_table.clone(),
            Expr::tbl(update_user_table, IamAccount::Id).equals(IamAccountRole::Table, IamAccountRole::UpdateUser),
        )
        .and_where(Expr::tbl(IamAccountRole::Table, IamAccountRole::RelAccountId).eq(account_id))
        .order_by(IamAccountRole::UpdateTime, Order::Desc)
        .done();
    let items = BIOSFuns::reldb().fetch_all::<AccountRoleDetailResp>(&sql_builder, None).await?;
    BIOSRespHelper::ok(items)
}

#[delete("/console/app/account/{account_id}/role/{role_id}")]
pub async fn delete_account_role(req: HttpRequest) -> BIOSResp {
    let ident_info = get_ident_account_info(&req)?;
    let account_id: String = req.match_info().get("account_id").unwrap().parse()?;
    let role_id: String = req.match_info().get("role_id").unwrap().parse()?;

    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccount::Id])
                .from(IamAccount::Table)
                .and_where(Expr::col(IamAccount::Id).eq(account_id.as_str()))
                .and_where(Expr::col(IamAccount::RelTenantId).eq(ident_info.tenant_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Account not exists".to_string()));
    }
    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamRole::Id])
                .from(IamRole::Table)
                .and_where(Expr::col(IamRole::Id).eq(role_id.as_str()))
                .and_where(Expr::col(IamRole::RelAppId).eq(ident_info.app_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Role not exists".to_string()));
    }

    let mut conn = BIOSFuns::reldb().conn().await;
    let mut tx = conn.begin().await?;

    let sql_builder = Query::select()
        .columns(IamAccountRole::iter().filter(|i| *i != IamAccountRole::Table))
        .from(IamAccountRole::Table)
        .and_where(Expr::col(IamAccountRole::RelAccountId).eq(account_id.as_str()))
        .and_where(Expr::col(IamAccountRole::RelRoleId).eq(role_id))
        .done();
    BIOSFuns::reldb().soft_del(IamAccountRole::Table, IamAccountRole::Id, &ident_info.account_id, &sql_builder, &mut tx).await?;
    // Remove token
    cache_processor::remove_token_by_account(&account_id).await?;
    tx.commit().await?;
    BIOSRespHelper::ok("")
}

// ------------------------------------

#[post("/console/app/account/{account_id}/group/{group_node_id}")]
pub async fn add_account_group(req: HttpRequest) -> BIOSResp {
    let ident_info = get_ident_account_info(&req)?;
    let account_id: String = req.match_info().get("account_id").unwrap().parse()?;
    let group_node_id: String = req.match_info().get("group_node_id").unwrap().parse()?;
    let id = bios::basic::field::uuid();

    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccount::Id])
                .from(IamAccount::Table)
                .and_where(Expr::col(IamAccount::Id).eq(account_id.as_str()))
                .and_where(Expr::col(IamAccount::RelTenantId).eq(ident_info.tenant_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Account not exists".to_string()));
    }
    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![(IamGroupNode::Table, IamGroupNode::Id)])
                .from(IamGroupNode::Table)
                .inner_join(
                    IamGroup::Table,
                    Expr::tbl(IamGroup::Table, IamGroup::Id).equals(IamGroupNode::Table, IamGroupNode::RelGroupId),
                )
                .and_where(Expr::tbl(IamGroupNode::Table, IamGroupNode::Id).eq(group_node_id.as_str()))
                .and_where(Expr::tbl(IamGroup::Table, IamGroup::RelAppId).eq(ident_info.app_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("GroupNode not exists".to_string()));
    }

    if BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccountGroup::Id])
                .from(IamAccountGroup::Table)
                .and_where(Expr::col(IamAccountGroup::RelGroupNodeId).eq(group_node_id.as_str()))
                .and_where(Expr::col(IamAccountGroup::RelAccountId).eq(account_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return Err(BIOSError::Conflict("IamAccountGroup [rel_group_node_id] and [rel_account_id] already exists".to_string()));
    }

    BIOSFuns::reldb()
        .exec(
            &Query::insert()
                .into_table(IamAccountGroup::Table)
                .columns(vec![
                    IamAccountGroup::Id,
                    IamAccountGroup::CreateUser,
                    IamAccountGroup::UpdateUser,
                    IamAccountGroup::RelAccountId,
                    IamAccountGroup::RelGroupNodeId,
                ])
                .values_panic(vec![
                    id.as_str().into(),
                    ident_info.account_id.as_str().into(),
                    ident_info.account_id.as_str().into(),
                    account_id.into(),
                    group_node_id.into(),
                ])
                .done(),
            None,
        )
        .await?;
    BIOSRespHelper::ok(id)
}

#[get("/console/app/account/{account_id}/group")]
pub async fn list_account_group(req: HttpRequest) -> BIOSResp {
    let ident_info = get_ident_account_info(&req)?;
    let account_id: String = req.match_info().get("account_id").unwrap().parse()?;

    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccount::Id])
                .from(IamAccount::Table)
                .and_where(Expr::col(IamAccount::Id).eq(account_id.as_str()))
                .and_where(Expr::col(IamAccount::RelTenantId).eq(ident_info.tenant_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Account not exists".to_string()));
    }

    let create_user_table = Alias::new("create");
    let update_user_table = Alias::new("update");
    let sql_builder = Query::select()
        .columns(vec![
            (IamAccountGroup::Table, IamAccountGroup::Id),
            (IamAccountGroup::Table, IamAccountGroup::CreateTime),
            (IamAccountGroup::Table, IamAccountGroup::UpdateTime),
            (IamAccountGroup::Table, IamAccountGroup::RelAccountId),
            (IamAccountGroup::Table, IamAccountGroup::RelGroupNodeId),
        ])
        .expr_as(Expr::tbl(create_user_table.clone(), IamAccount::Name), Alias::new("create_user"))
        .expr_as(Expr::tbl(update_user_table.clone(), IamAccount::Name), Alias::new("update_user"))
        .from(IamAccountGroup::Table)
        .join_as(
            JoinType::InnerJoin,
            IamAccount::Table,
            create_user_table.clone(),
            Expr::tbl(create_user_table, IamAccount::Id).equals(IamAccountGroup::Table, IamAccountGroup::CreateUser),
        )
        .join_as(
            JoinType::InnerJoin,
            IamAccount::Table,
            update_user_table.clone(),
            Expr::tbl(update_user_table, IamAccount::Id).equals(IamAccountGroup::Table, IamAccountGroup::UpdateUser),
        )
        .and_where(Expr::tbl(IamAccountGroup::Table, IamAccountGroup::RelAccountId).eq(account_id))
        .order_by(IamAccountGroup::UpdateTime, Order::Desc)
        .done();
    let items = BIOSFuns::reldb().fetch_all::<AccountGroupDetailResp>(&sql_builder, None).await?;
    BIOSRespHelper::ok(items)
}

#[delete("/console/app/account/{account_id}/group/{group_node_id}")]
pub async fn delete_account_group(req: HttpRequest) -> BIOSResp {
    let ident_info = get_ident_account_info(&req)?;
    let account_id: String = req.match_info().get("account_id").unwrap().parse()?;
    let group_node_id: String = req.match_info().get("group_node_id").unwrap().parse()?;

    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![IamAccount::Id])
                .from(IamAccount::Table)
                .and_where(Expr::col(IamAccount::Id).eq(account_id.as_str()))
                .and_where(Expr::col(IamAccount::RelTenantId).eq(ident_info.tenant_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("Account not exists".to_string()));
    }
    if !BIOSFuns::reldb()
        .exists(
            &Query::select()
                .columns(vec![(IamGroupNode::Table, IamGroupNode::Id)])
                .from(IamGroupNode::Table)
                .inner_join(
                    IamGroup::Table,
                    Expr::tbl(IamGroup::Table, IamGroup::Id).equals(IamGroupNode::Table, IamGroupNode::RelGroupId),
                )
                .and_where(Expr::tbl(IamGroupNode::Table, IamGroupNode::Id).eq(group_node_id.as_str()))
                .and_where(Expr::tbl(IamGroup::Table, IamGroup::RelAppId).eq(ident_info.app_id.as_str()))
                .done(),
            None,
        )
        .await?
    {
        return BIOSRespHelper::bus_error(BIOSError::NotFound("GroupNode not exists".to_string()));
    }

    let mut conn = BIOSFuns::reldb().conn().await;
    let mut tx = conn.begin().await?;

    let sql_builder = Query::select()
        .columns(IamAccountGroup::iter().filter(|i| *i != IamAccountGroup::Table))
        .from(IamAccountGroup::Table)
        .and_where(Expr::col(IamAccountGroup::RelAccountId).eq(account_id.as_str()))
        .and_where(Expr::col(IamAccountGroup::RelGroupNodeId).eq(group_node_id))
        .done();
    BIOSFuns::reldb().soft_del(IamAccountGroup::Table, IamAccountGroup::Id, &ident_info.account_id, &sql_builder, &mut tx).await?;
    // Remove token
    cache_processor::remove_token_by_account(&account_id).await?;
    tx.commit().await?;
    BIOSRespHelper::ok("")
}