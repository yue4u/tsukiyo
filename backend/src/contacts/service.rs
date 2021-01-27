use super::model::*;
use crate::sql::schema::contacts::{self, dsl::*};
use crate::Context;
use diesel::prelude::*;
use validator::Validate;

pub fn get(ctx: &Context, contact_id: i32) -> anyhow::Result<Contact> {
    let conn = ctx.pool.get()?;
    Ok(contacts.filter(id.eq(contact_id)).get_result(&conn)?)
}

pub fn create(ctx: &Context, contact: ContactInput) -> anyhow::Result<Contact> {
    contact.validate()?;
    let notify = contact.clone().to_notify_message();
    actix_rt::spawn(async move {
        actix_web::client::Client::default()
            .post(std::env::var("CONTACT_WEB_HOOK").unwrap())
            .send_json(&notify)
            .await
            .unwrap();
    });
    let conn = ctx.pool.get()?;
    Ok(diesel::insert_into(contacts::table)
        .values(&contact)
        .get_result::<Contact>(&conn)?)
}

pub fn update(ctx: &Context, contact_id: i32, contact: ContactUpdate) -> anyhow::Result<Contact> {
    let conn = ctx.pool.get()?;
    Ok(diesel::update(contacts.filter(id.eq(contact_id)))
        .set(&contact)
        .get_result(&conn)?)
}

pub fn delete(ctx: &Context, contact_id: i32) -> anyhow::Result<Contact> {
    let conn = ctx.pool.get()?;
    Ok(diesel::delete(contacts.filter(id.eq(contact_id))).get_result(&conn)?)
}

pub fn list(ctx: &Context, by: Option<ContactQuery>) -> anyhow::Result<Vec<Contact>> {
    let conn = ctx.pool.get()?;
    let mut query = contacts::table.into_boxed();
    if let Some(by_contact) = by {
        // TODO: simplify this
        // if let Some(_published) = by_contact.search_string {
        // query = query.filter(contacts::published.eq(_published));
        // }
        if let Some(_checked) = by_contact.checked {
            query = query.filter(contacts::checked.eq(_checked));
        }
    }
    Ok(query.load::<Contact>(&conn)?)
}
