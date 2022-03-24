use tardis::basic::dto::TardisContext;
use tardis::basic::result::TardisResult;
use tardis::db::reldb_client::TardisRelDBlConnection;
use tardis::web::web_resp::TardisPage;

use bios_basic::rbum::dto::filer_dto::RbumItemFilterReq;
use bios_basic::rbum::dto::rbum_rel_agg_dto::RbumRelAggResp;
use bios_basic::rbum::serv::rbum_item_serv::RbumItemCrudOperation;

use crate::basic::constants;
use crate::basic::dto::iam_role_dto::{IamRoleAddReq, IamRoleDetailResp, IamRoleModifyReq, IamRoleSummaryResp};
use crate::basic::enumeration::IAMRelKind;
use crate::basic::serv::iam_rel_serv::IamRelServ;
use crate::basic::serv::iam_role_serv::IamRoleServ;
use crate::basic::serv::iam_tenant_serv::IamTenantServ;
use crate::console_tenant::dto::iam_ct_role_dto::{IamCtRoleAddReq, IamCtRoleModifyReq};

pub struct IamCtRoleServ;

impl<'a> IamCtRoleServ {
    pub async fn add_role(add_req: &mut IamCtRoleAddReq, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<String> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRoleServ::add_item_with_simple_rel(
            &mut IamRoleAddReq {
                name: add_req.name.clone(),
                icon: add_req.icon.clone(),
                disabled: add_req.disabled,
                scope_level: constants::RBUM_SCOPE_LEVEL_TENANT,
                sort: add_req.sort,
            },
            &IAMRelKind::IamRoleTenant.to_string(),
            &IamTenantServ::get_id_by_cxt(cxt)?,
            db,
            cxt,
        )
        .await
    }

    pub async fn modify_role(id: &str, modify_req: &mut IamCtRoleModifyReq, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<()> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRoleServ::modify_item(
            id,
            &mut IamRoleModifyReq {
                name: modify_req.name.clone(),
                icon: modify_req.icon.clone(),
                disabled: modify_req.disabled,
                scope_level: None,
                sort: modify_req.sort,
            },
            db,
            cxt,
        )
        .await
    }

    pub async fn get_role(id: &str, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<IamRoleDetailResp> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRoleServ::get_item(id, &RbumItemFilterReq::default(), db, cxt).await
    }

    pub async fn paginate_roles(
        q_name: Option<String>,
        page_number: u64,
        page_size: u64,
        desc_sort_by_create: Option<bool>,
        desc_sort_by_update: Option<bool>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<TardisPage<IamRoleSummaryResp>> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRoleServ::paginate_items(
            &RbumItemFilterReq {
                name: q_name,
                rel_scope_paths: Some(IamTenantServ::get_id_by_cxt(cxt)?),
                ..Default::default()
            },
            page_number,
            page_size,
            desc_sort_by_create,
            desc_sort_by_update,
            db,
            cxt,
        )
        .await
    }

    pub async fn delete_role(id: &str, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<u64> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRoleServ::delete_item(id, db, cxt).await
    }

    pub async fn add_rel_account(
        iam_role_id: &str,
        iam_account_id: &str,
        start_timestamp: Option<i64>,
        end_timestamp: Option<i64>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<()> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRelServ::add_rel(IAMRelKind::IamRoleAccount, iam_role_id, iam_account_id, start_timestamp, end_timestamp, db, cxt).await
    }

    pub async fn paginate_rel_accounts(
        iam_role_id: &str,
        page_number: u64,
        page_size: u64,
        desc_sort_by_create: Option<bool>,
        desc_sort_by_update: Option<bool>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<TardisPage<RbumRelAggResp>> {
        IamRelServ::paginate_from_rels(
            IAMRelKind::IamRoleAccount,
            iam_role_id,
            page_number,
            page_size,
            desc_sort_by_create,
            desc_sort_by_update,
            db,
            cxt,
        )
        .await
    }

    pub async fn delete_rel_account(iam_role_id: &str, iam_account_id: &str, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<()> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRelServ::delete_rel(IAMRelKind::IamRoleAccount, iam_role_id, iam_account_id, db, cxt).await
    }

    pub async fn add_rel_http_res(
        iam_role_id: &str,
        iam_http_res_id: &str,
        start_timestamp: Option<i64>,
        end_timestamp: Option<i64>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<()> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRelServ::add_rel(IAMRelKind::IamRoleHttpRes, iam_role_id, iam_http_res_id, start_timestamp, end_timestamp, db, cxt).await
    }

    pub async fn paginate_rel_http_res(
        iam_role_id: &str,
        page_number: u64,
        page_size: u64,
        desc_sort_by_create: Option<bool>,
        desc_sort_by_update: Option<bool>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<TardisPage<RbumRelAggResp>> {
        IamRelServ::paginate_from_rels(
            IAMRelKind::IamRoleHttpRes,
            iam_role_id,
            page_number,
            page_size,
            desc_sort_by_create,
            desc_sort_by_update,
            db,
            cxt,
        )
        .await
    }

    pub async fn delete_rel_http_res(iam_role_id: &str, iam_http_res_id: &str, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<()> {
        IamRoleServ::need_tenant_admin(db, cxt).await?;
        IamRelServ::delete_rel(IAMRelKind::IamRoleHttpRes, iam_role_id, iam_http_res_id, db, cxt).await
    }
}