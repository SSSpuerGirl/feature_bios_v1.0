use tardis::basic::dto::{TardisContext, TardisFunsInst};
use tardis::basic::field::TrimString;
use tardis::basic::result::TardisResult;
use tardis::log::info;

use bios_basic::rbum::dto::rbum_filer_dto::RbumBasicFilterReq;
use bios_basic::rbum::serv::rbum_item_serv::RbumItemCrudOperation;
use bios_iam::basic::dto::iam_account_dto::{IamAccountAddReq, IamAccountModifyReq};
use bios_iam::basic::dto::iam_filer_dto::IamAccountFilterReq;
use bios_iam::basic::dto::iam_role_dto::IamRoleAddReq;
use bios_iam::basic::serv::iam_account_serv::IamAccountServ;
use bios_iam::basic::serv::iam_role_serv::IamRoleServ;
use bios_iam::iam_constants;
use bios_iam::iam_constants::{RBUM_ITEM_NAME_APP_ADMIN_ROLE, RBUM_ITEM_NAME_SYS_ADMIN_ACCOUNT, RBUM_ITEM_NAME_SYS_ADMIN_ROLE, RBUM_ITEM_NAME_TENANT_ADMIN_ROLE};

pub async fn test(
    sys_context: &TardisContext,
    t1_context: &TardisContext,
    t2_context: &TardisContext,
    t2_a1_context: &TardisContext,
    t2_a2_context: &TardisContext,
) -> TardisResult<()> {
    test_single_level(sys_context, RBUM_ITEM_NAME_SYS_ADMIN_ACCOUNT, RBUM_ITEM_NAME_SYS_ADMIN_ROLE, t1_context).await?;
    test_single_level(t1_context, "测试管理员1", RBUM_ITEM_NAME_TENANT_ADMIN_ROLE, t2_context).await?;
    test_single_level(t2_a1_context, "应用1管理员", RBUM_ITEM_NAME_APP_ADMIN_ROLE, t2_a2_context).await?;
    test_multi_level_by_sys_context(sys_context, t1_context, t2_context, t2_a1_context, t2_a2_context).await?;
    test_multi_level_by_tenant_context(sys_context, t1_context, t2_context, t2_a1_context, t2_a2_context).await?;
    test_multi_level_by_app_context(sys_context, t1_context, t2_context, t2_a1_context, t2_a2_context).await?;
    Ok(())
}

async fn test_single_level(context: &TardisContext, account_name: &str, role_name: &str, another_context: &TardisContext) -> TardisResult<()> {
    let mut funs = iam_constants::get_tardis_inst();
    funs.begin().await?;
    info!("【test_cc_account】 : test_single_level : Prepare : Add Role");
    let role_id = IamRoleServ::add_item(
        &mut IamRoleAddReq {
            name: TrimString("角色1".to_string()),
            icon: None,
            sort: None,
            disabled: None,
            scope_level: None,
        },
        &funs,
        context,
    )
    .await?;

    info!("【test_cc_account】 : test_single_level : Add Account");
    let account_id = IamAccountServ::add_item(
        &mut IamAccountAddReq {
            id: None,
            name: TrimString("星航1".to_string()),
            icon: None,
            scope_level: None,
            disabled: None,
        },
        &funs,
        context,
    )
    .await?;

    info!("【test_cc_account】 : test_single_level : Modify Account By Id");
    assert!(IamAccountServ::modify_item(
        &account_id,
        &mut IamAccountModifyReq {
            name: Some(TrimString("星航3".to_string())),
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None
        },
        &funs,
        another_context
    )
    .await
    .is_err());
    IamAccountServ::modify_item(
        &account_id,
        &mut IamAccountModifyReq {
            name: Some(TrimString("星航".to_string())),
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        context,
    )
    .await?;

    info!("【test_cc_account】 : test_single_level : Get Account By Id");
    assert!(IamAccountServ::get_item(&account_id, &IamAccountFilterReq::default(), &funs, another_context).await.is_err());
    let account = IamAccountServ::get_item(&account_id, &IamAccountFilterReq::default(), &funs, context).await?;
    assert_eq!(account.id, account_id);
    assert_eq!(account.name, "星航");
    assert_eq!(account.icon, "/icon/icon.png");
    assert!(!account.disabled);

    info!("【test_cc_account】 : test_single_level : Find Accounts");
    let accounts = IamAccountServ::paginate_items(&IamAccountFilterReq::default(), 1, 10, None, None, &funs, context).await?;
    assert_eq!(accounts.page_number, 1);
    assert_eq!(accounts.page_size, 10);
    assert!(accounts.records.iter().any(|i| i.name == "星航"));

    info!("【test_cc_account】 : test_single_level : Find Rel Roles By Account Id");
    let account_roles = IamAccountServ::paginate_rel_roles(&account_id, 1, 10, None, None, &funs, another_context).await?;
    assert_eq!(account_roles.page_number, 1);
    assert_eq!(account_roles.page_size, 10);
    assert_eq!(account_roles.total_size, 0);
    let account_roles = IamAccountServ::paginate_rel_roles(&context.owner, 1, 10, None, None, &funs, context).await?;
    assert_eq!(account_roles.page_number, 1);
    assert_eq!(account_roles.page_size, 10);
    assert_eq!(account_roles.total_size, 1);
    assert_eq!(account_roles.records.len(), 1);
    assert_eq!(account_roles.records.get(0).unwrap().rel.from_rbum_item_name, account_name);
    assert_eq!(account_roles.records.get(0).unwrap().rel.to_rbum_item_name, role_name);

    info!("【test_cc_account】 : test_single_level : Add Rel Account By Id");
    assert!(IamRoleServ::add_rel_account(&role_id, &account_id, &funs, another_context).await.is_err());
    IamRoleServ::add_rel_account(&role_id, &account_id, &funs, context).await?;
    let account_roles = IamAccountServ::paginate_rel_roles(&account_id, 1, 10, None, None, &funs, context).await?;
    assert_eq!(account_roles.page_number, 1);
    assert_eq!(account_roles.page_size, 10);
    assert_eq!(account_roles.total_size, 1);

    info!("【test_cc_account】 : test_single_level : Delete Account By Id");
    assert!(IamAccountServ::delete_item_with_all_rels("11111", &funs, context).await.is_err());
    assert!(IamAccountServ::delete_item_with_all_rels(&account_id, &funs, another_context).await.is_err());
    IamAccountServ::delete_item_with_all_rels(&account_id, &funs, context).await?;
    assert_eq!(
        IamAccountServ::paginate_items(
            &IamAccountFilterReq {
                basic: RbumBasicFilterReq {
                    ids: Some(vec![account_id]),
                    ..Default::default()
                },
                ..Default::default()
            },
            1,
            10,
            None,
            None,
            &funs,
            context
        )
        .await?
        .total_size,
        0
    );

    funs.rollback().await?;
    Ok(())
}

async fn test_multi_level_add<'a>(
    sys_context: &TardisContext,
    t1_context: &TardisContext,
    t2_context: &TardisContext,
    t2_a1_context: &TardisContext,
    t2_a2_context: &TardisContext,
    funs: &TardisFunsInst<'a>,
) -> TardisResult<(String, String, String, String, String)> {
    info!("【test_cc_account】 : test_multi_level : Add Account");

    let account_sys_id = IamAccountServ::add_item(
        &mut IamAccountAddReq {
            id: None,
            name: TrimString("account_sys".to_string()),
            icon: None,
            scope_level: None,
            disabled: None,
        },
        funs,
        sys_context,
    )
    .await?;

    let account_t1_id = IamAccountServ::add_item(
        &mut IamAccountAddReq {
            id: None,
            name: TrimString("account_t1".to_string()),
            icon: None,
            scope_level: None,
            disabled: None,
        },
        funs,
        t1_context,
    )
    .await?;

    let account_t2_id = IamAccountServ::add_item(
        &mut IamAccountAddReq {
            id: None,
            name: TrimString("account_t2".to_string()),
            icon: None,
            scope_level: None,
            disabled: None,
        },
        funs,
        t2_context,
    )
    .await?;

    let account_t2_a1_id = IamAccountServ::add_item(
        &mut IamAccountAddReq {
            id: None,
            name: TrimString("account_t2_a1".to_string()),
            icon: None,
            scope_level: None,
            disabled: None,
        },
        funs,
        t2_a1_context,
    )
    .await?;

    let account_t2_a2_id = IamAccountServ::add_item(
        &mut IamAccountAddReq {
            id: None,
            name: TrimString("account_t2_a2".to_string()),
            icon: None,
            scope_level: None,
            disabled: None,
        },
        funs,
        t2_a2_context,
    )
    .await?;

    Ok((account_sys_id, account_t1_id, account_t2_id, account_t2_a1_id, account_t2_a2_id))
}

pub async fn test_multi_level_by_sys_context(
    sys_context: &TardisContext,
    t1_context: &TardisContext,
    t2_context: &TardisContext,
    t2_a1_context: &TardisContext,
    t2_a2_context: &TardisContext,
) -> TardisResult<()> {
    let mut funs = iam_constants::get_tardis_inst();
    funs.begin().await?;

    let (account_sys_id, account_t1_id, account_t2_id, account_t2_a1_id, account_t2_a2_id) =
        test_multi_level_add(sys_context, t1_context, t2_context, t2_a1_context, t2_a2_context, &funs).await?;

    info!("【test_cc_account】 : test_multi_level : Modify Account By sys_context");
    IamAccountServ::modify_item(
        &account_sys_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        sys_context,
    )
    .await?;
    IamAccountServ::modify_item(
        &account_t1_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        sys_context,
    )
    .await?;
    IamAccountServ::modify_item(
        &account_t2_a1_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        sys_context,
    )
    .await?;
    info!("【test_cc_account】 : test_multi_level : Get Account By sys_context");
    assert_eq!(
        IamAccountServ::get_item(&account_sys_id, &IamAccountFilterReq::default(), &funs, sys_context).await?.name,
        "account_sys"
    );
    assert!(IamAccountServ::get_item(&account_t1_id, &IamAccountFilterReq::default(), &funs, sys_context).await.is_err());
    assert_eq!(
        IamAccountServ::get_item(
            &account_t1_id,
            &IamAccountFilterReq {
                basic: RbumBasicFilterReq {
                    own_paths: Some(sys_context.own_paths.clone()),
                    with_sub_own_paths: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            &funs,
            sys_context
        )
        .await?
        .name,
        "account_t1"
    );
    assert!(IamAccountServ::get_item(&account_t2_a1_id, &IamAccountFilterReq::default(), &funs, sys_context).await.is_err());
    assert_eq!(
        IamAccountServ::get_item(
            &account_t2_a1_id,
            &IamAccountFilterReq {
                basic: RbumBasicFilterReq {
                    own_paths: Some(sys_context.own_paths.clone()),
                    with_sub_own_paths: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            &funs,
            sys_context
        )
        .await?
        .name,
        "account_t2_a1"
    );

    info!("【test_cc_account】 : test_multi_level : Delete Account By sys_context");
    IamAccountServ::delete_item_with_all_rels(&account_sys_id, &funs, sys_context).await?;
    IamAccountServ::delete_item_with_all_rels(&account_t1_id, &funs, sys_context).await?;
    IamAccountServ::delete_item_with_all_rels(&account_t2_a1_id, &funs, sys_context).await?;

    funs.rollback().await?;
    Ok(())
}

pub async fn test_multi_level_by_tenant_context(
    sys_context: &TardisContext,
    t1_context: &TardisContext,
    t2_context: &TardisContext,
    t2_a1_context: &TardisContext,
    t2_a2_context: &TardisContext,
) -> TardisResult<()> {
    let mut funs = iam_constants::get_tardis_inst();
    funs.begin().await?;

    let (account_sys_id, account_t1_id, account_t2_id, account_t2_a1_id, account_t2_a2_id) =
        test_multi_level_add(sys_context, t1_context, t2_context, t2_a1_context, t2_a2_context, &funs).await?;

    info!("【test_cc_account】 : test_multi_level : Modify Account By tenant_context");
    assert!(IamAccountServ::modify_item(
        &account_sys_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t1_context,
    )
    .await
    .is_err());
    assert!(IamAccountServ::modify_item(
        &account_t1_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_context,
    )
    .await
    .is_err());
    IamAccountServ::modify_item(
        &account_t2_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_context,
    )
    .await?;
    IamAccountServ::modify_item(
        &account_t2_a1_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_context,
    )
    .await?;
    info!("【test_cc_account】 : test_multi_level : Get Account By tenant_context");
    assert!(IamAccountServ::get_item(&account_sys_id, &IamAccountFilterReq::default(), &funs, t2_context).await.is_err());
    assert!(IamAccountServ::get_item(&account_t1_id, &IamAccountFilterReq::default(), &funs, t2_context).await.is_err());
    assert_eq!(
        IamAccountServ::get_item(&account_t2_id, &IamAccountFilterReq::default(), &funs, t2_context).await?.name,
        "account_t2"
    );
    assert!(IamAccountServ::get_item(&account_t2_a1_id, &IamAccountFilterReq::default(), &funs, t2_context).await.is_err());
    assert_eq!(
        IamAccountServ::get_item(
            &account_t2_a1_id,
            &IamAccountFilterReq {
                basic: RbumBasicFilterReq {
                    own_paths: Some(t2_context.own_paths.clone()),
                    with_sub_own_paths: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            &funs,
            t2_context
        )
        .await?
        .name,
        "account_t2_a1"
    );

    info!("【test_cc_account】 : test_multi_level : Delete Account By tenant_context");
    assert!(IamAccountServ::delete_item_with_all_rels(&account_sys_id, &funs, t2_context).await.is_err());
    assert!(IamAccountServ::delete_item_with_all_rels(&account_t1_id, &funs, t2_context).await.is_err());
    IamAccountServ::delete_item_with_all_rels(&account_t2_id, &funs, t2_context).await?;
    IamAccountServ::delete_item_with_all_rels(&account_t2_a1_id, &funs, t2_context).await?;

    funs.rollback().await?;
    Ok(())
}

pub async fn test_multi_level_by_app_context(
    sys_context: &TardisContext,
    t1_context: &TardisContext,
    t2_context: &TardisContext,
    t2_a1_context: &TardisContext,
    t2_a2_context: &TardisContext,
) -> TardisResult<()> {
    let mut funs = iam_constants::get_tardis_inst();
    funs.begin().await?;

    let (account_sys_id, account_t1_id, account_t2_id, account_t2_a1_id, account_t2_a2_id) =
        test_multi_level_add(sys_context, t1_context, t2_context, t2_a1_context, t2_a2_context, &funs).await?;

    info!("【test_cc_account】 : test_multi_level : Modify Account By app_context");
    assert!(IamAccountServ::modify_item(
        &account_sys_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_a1_context,
    )
    .await
    .is_err());
    assert!(IamAccountServ::modify_item(
        &account_t1_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_a1_context,
    )
    .await
    .is_err());
    assert!(IamAccountServ::modify_item(
        &account_t2_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_a1_context,
    )
    .await
    .is_err());
    assert!(IamAccountServ::modify_item(
        &account_t2_a2_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_a1_context,
    )
    .await
    .is_err());
    IamAccountServ::modify_item(
        &account_t2_a1_id,
        &mut IamAccountModifyReq {
            name: None,
            icon: Some("/icon/icon.png".to_string()),
            scope_level: None,
            disabled: None,
        },
        &funs,
        t2_a1_context,
    )
    .await?;

    info!("【test_cc_account】 : test_multi_level : Get Account By app_context");
    assert!(IamAccountServ::get_item(&account_sys_id, &IamAccountFilterReq::default(), &funs, t2_a1_context).await.is_err());
    assert!(IamAccountServ::get_item(&account_t1_id, &IamAccountFilterReq::default(), &funs, t2_a1_context).await.is_err());
    assert!(IamAccountServ::get_item(&account_t2_id, &IamAccountFilterReq::default(), &funs, t2_a1_context).await.is_err());
    assert_eq!(
        IamAccountServ::get_item(&account_t2_a1_id, &IamAccountFilterReq::default(), &funs, t2_a1_context).await?.name,
        "account_t2_a1"
    );

    info!("【test_cc_account】 : test_multi_level : Delete Account By app_context");
    assert!(IamAccountServ::delete_item_with_all_rels(&account_sys_id, &funs, t2_a1_context).await.is_err());
    assert!(IamAccountServ::delete_item_with_all_rels(&account_t1_id, &funs, t2_a1_context).await.is_err());
    assert!(IamAccountServ::delete_item_with_all_rels(&account_t2_id, &funs, t2_a1_context).await.is_err());
    IamAccountServ::delete_item_with_all_rels(&account_t2_a1_id, &funs, t2_a1_context).await?;

    funs.rollback().await?;
    Ok(())
}