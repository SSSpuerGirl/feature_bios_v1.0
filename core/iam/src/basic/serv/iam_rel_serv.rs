use tardis::basic::dto::TardisContext;
use tardis::basic::result::TardisResult;
use tardis::db::reldb_client::TardisRelDBlConnection;
use tardis::web::web_resp::TardisPage;

use bios_basic::rbum::dto::rbum_rel_agg_dto::{RbumRelAggAddReq, RbumRelAggResp, RbumRelEnvAggAddReq};
use bios_basic::rbum::dto::rbum_rel_dto::{RbumRelAddReq, RbumRelFindReq};
use bios_basic::rbum::rbum_enumeration::RbumRelEnvKind;
use bios_basic::rbum::serv::rbum_crud_serv::RbumCrudOperation;
use bios_basic::rbum::serv::rbum_rel_serv::RbumRelServ;

use crate::basic::enumeration::IAMRelKind;

pub struct IamRelServ;

impl<'a> IamRelServ {
    pub async fn add_rel(
        rel_kind: IAMRelKind,
        from_iam_item_id: &str,
        to_iam_item_id: &str,
        start_timestamp: Option<i64>,
        end_timestamp: Option<i64>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<()> {
        let req = &mut RbumRelAggAddReq {
            rel: RbumRelAddReq {
                tag: rel_kind.to_string(),
                from_rbum_item_id: from_iam_item_id.to_string(),
                to_rbum_item_id: to_iam_item_id.to_string(),
                to_scope_paths: cxt.scope_paths.to_string(),
                ext: None,
            },
            attrs: vec![],
            envs: if start_timestamp.is_some() || end_timestamp.is_some() {
                vec![RbumRelEnvAggAddReq {
                    kind: RbumRelEnvKind::DatetimeRange,
                    value1: start_timestamp.unwrap_or(i64::MAX).to_string(),
                    value2: Some(end_timestamp.unwrap_or(i64::MAX).to_string()),
                }]
            } else {
                vec![]
            },
        };
        RbumRelServ::add_rel(req, db, cxt).await?;
        Ok(())
    }

    pub async fn paginate_from_rels(
        rel_kind: IAMRelKind,
        from_iam_item_id: &str,
        page_number: u64,
        page_size: u64,
        desc_sort_by_create: Option<bool>,
        desc_sort_by_update: Option<bool>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<TardisPage<RbumRelAggResp>> {
        RbumRelServ::paginate_from_rels(
            &rel_kind.to_string(),
            from_iam_item_id,
            page_number,
            page_size,
            desc_sort_by_create,
            desc_sort_by_update,
            db,
            cxt,
        )
        .await
    }

    pub async fn paginate_to_rels(
        rel_kind: IAMRelKind,
        to_iam_item_id: &str,
        page_number: u64,
        page_size: u64,
        desc_sort_by_create: Option<bool>,
        desc_sort_by_update: Option<bool>,
        db: &TardisRelDBlConnection<'a>,
        cxt: &TardisContext,
    ) -> TardisResult<TardisPage<RbumRelAggResp>> {
        RbumRelServ::paginate_to_rels(
            &rel_kind.to_string(),
            to_iam_item_id,
            page_number,
            page_size,
            desc_sort_by_create,
            desc_sort_by_update,
            db,
            cxt,
        )
        .await
    }

    pub async fn delete_rel(rel_kind: IAMRelKind, from_iam_item_id: &str, to_iam_item_id: &str, db: &TardisRelDBlConnection<'a>, cxt: &TardisContext) -> TardisResult<()> {
        let id = RbumRelServ::find_rel_id(
            &RbumRelFindReq {
                tag: rel_kind.to_string(),
                from_rbum_item_id: from_iam_item_id.to_string(),
                to_rbum_item_id: to_iam_item_id.to_string(),
            },
            db,
            cxt,
        )
        .await?;
        if let Some(id) = id {
            RbumRelServ::delete_rbum(&id, db, cxt).await?;
        }
        Ok(())
    }
}