//! Leave a fresh, genuinely approved two-line order for the mounted Rules proof.
use super::{saved, show};
use bilibi_database::Result;
use bilibi_supply_chain_purchasing::{
    NewOrder, NewOrderLine, NewSupplier, OrderAmendment, Purchasing,
};
use bilibi_tenancy::Scope;
pub(super) async fn prepare(purchasing: &Purchasing, manager: &Scope, admin: &Scope) -> Result<()> {
    let supplier = saved(show(
        "rule source supplier",
        purchasing
            .create_supplier(
                manager,
                1,
                &NewSupplier {
                    code: "RULE-FOLLOWUP".into(),
                    name: "Follow-up supplier".into(),
                    ..NewSupplier::default()
                },
            )
            .await,
    ))?;
    let line = NewOrderLine {
        description: "Follow-up proof".into(),
        external_item_ref: String::new(),
        quantity: "1".into(),
        unit: "item".into(),
        unit_price: "10".into(),
        tax_code_id: None,
    };
    let order = saved(show(
        "rule source order",
        purchasing
            .create_order(
                manager,
                1,
                &NewOrder {
                    supplier_id: Some(supplier.id),
                    currency_code: "MYR".into(),
                    lines: vec![line.clone(), line],
                    ..NewOrder::default()
                },
            )
            .await,
    ))?;
    saved(show(
        "submit rule source",
        purchasing.submit_order(manager, 1, order.id).await,
    ))?;
    saved(show(
        "independently approve rule source",
        purchasing
            .approve_order(admin, 1, order.id, "Follow-up proof")
            .await,
    ))?;
    Ok(())
}

/// Prepare only the source records needed by the follow-up proof.
pub async fn prepare_rules(url: &str, mode: &str, order_id: Option<i64>) -> Result<()> {
    use bilibi_module_registry::Services;
    use bilibi_tenancy::Authentication;
    use std::sync::Arc;
    bilibi_base::link();
    bilibi_core::link();
    bilibi_domains::link();
    bilibi_extensions::link();
    let db = bilibi_database::pool(url)?;
    let modules = bilibi_module_registry::discover()?;
    if mode == "prepare" {
        bilibi_database::migrate(&db, &modules).await?;
        bilibi_walkthrough_fixture::install(&db).await?;
        bilibi_walkthrough_fixture::set_instance_role(&db, "development").await?;
    }
    let mut services = Services::default();
    services.provide("bootstrap.database", Arc::new(db.clone()))?;
    services.provide("bootstrap.composition", Arc::new(bilibi_web::composition()))?;
    services.install(&modules)?;
    if mode == "prepare" {
        bilibi_database::seeds::run(&db, &modules, services.catalog()?, &services).await?;
        let grants = services.get::<bilibi_authz::grants::Grants>("base/authz.grants")?;
        grants.reconcile().await?;
    }
    let auth = services.get::<Authentication>("base/tenancy.authentication")?;
    // Amendments keep the proof actor and its authority unchanged.
    if mode == "prepare" {
        let client = db.get().await?;
        let admin = auth
            .authenticate(&**client, 1, 4, "rules-source".into())
            .await?;
        let role = super::tenant_owner_role(&**client, &admin).await?;
        drop(client);
        let grants = services.get::<bilibi_authz::grants::Grants>("base/authz.grants")?;
        grants.assign_role(&admin, 1, role, true).await?;
        let person = services
            .get::<bilibi_person::PersonService>(bilibi_person::SERVICE)?
            .create(
                &admin,
                &bilibi_person::PersonInput {
                    name: "Rules proof administrator".into(),
                    kind: bilibi_person::PersonKind::Human,
                    email: None,
                    phone: None,
                },
            )
            .await
            .map_err(|_| bilibi_database::Error::Invalid)?;
        let users = services.get::<bilibi_user::UserService>("core/user.accounts")?;
        users
            .link_person(&admin, 4, person.id)
            .await
            .map_err(|_| bilibi_database::Error::Invalid)?;
        users.set_desk(&admin, 1, person.id.get(), true).await?;
    }
    let client = db.get().await?;
    let admin = auth
        .authenticate(&**client, 1, 4, "rules-source".into())
        .await?;
    let manager = auth
        .authenticate(&**client, 1, 1, "rules-source".into())
        .await?;
    drop(client);
    let purchasing = services.get::<Purchasing>(bilibi_supply_chain_purchasing::SERVICE)?;
    if mode == "prepare" {
        return prepare(&purchasing, &manager, &admin).await;
    }
    let id = order_id.ok_or(bilibi_database::Error::Invalid)?;
    let order = purchasing
        .get_order(&manager, 1, id)
        .await?
        .ok_or(bilibi_database::Error::NotFound)?;
    let mut lines: Vec<_> = order
        .lines
        .iter()
        .map(|line| NewOrderLine {
            description: line.description.clone(),
            external_item_ref: line.external_item_ref.clone().unwrap_or_default(),
            quantity: line.quantity.to_string(),
            unit: line.unit.clone(),
            unit_price: line.unit_price.to_string(),
            tax_code_id: line.tax_code_id,
        })
        .collect();
    if mode == "reapprove" {
        lines
            .first_mut()
            .ok_or(bilibi_database::Error::Invalid)?
            .description
            .push_str(" revised for a separate approval");
    } else if mode != "amend-header" {
        return Err(bilibi_database::Error::Invalid);
    }
    let amended = saved(show(
        "supported rules source amendment",
        purchasing
            .amend_order(
                &manager,
                1,
                id,
                &OrderAmendment {
                    reason: "Logical event identity walkthrough".into(),
                    reference: Some("Header amended between approval parts".into()),
                    notes: None,
                    expected_on: None,
                    lines,
                },
            )
            .await,
    ))?;
    if mode == "amend-header" {
        if amended.status.as_str() != "approved" {
            return Err(bilibi_database::Error::Invalid);
        }
    } else {
        if amended.status.as_str() != "submitted" {
            return Err(bilibi_database::Error::Invalid);
        }
        saved(show(
            "independently approve amended rule source",
            purchasing
                .approve_order(&admin, 1, id, "A distinct approval snapshot")
                .await,
        ))?;
    }
    Ok(())
}
