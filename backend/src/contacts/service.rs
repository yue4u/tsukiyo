use super::model::*;
use crate::sql::{
    db::Conn,
    schema::contacts::{self, dsl::*},
};
use crate::utils::MessageError;
use diesel::prelude::*;

pub fn get(conn: Conn, contact_id: i32) -> anyhow::Result<Contact> {
    Ok(contacts.filter(id.eq(contact_id)).get_result(&conn)?)
}

pub fn create(conn: Conn, contact: ContactInput) -> anyhow::Result<Contact> {
    if contact.title.is_empty() {
        return Err(MessageError::new("contact title should not be empty").into());
    }

    if contact.name.is_empty() {
        return Err(MessageError::new("contact name should not be empty").into());
    }

    if contact.email.is_empty() {
        return Err(MessageError::new("contact email should not be empty").into());
    }

    if contact.body.is_empty() {
        return Err(MessageError::new("contact body should not be empty").into());
    }

    Ok(diesel::insert_into(contacts::table)
        .values(&contact)
        .get_result::<Contact>(&conn)?)
}

pub fn update(conn: Conn, contact_id: i32, contact: ContactUpdate) -> anyhow::Result<Contact> {
    Ok(diesel::update(contacts.filter(id.eq(contact_id)))
        .set(&contact)
        .get_result(&conn)?)
}

pub fn delete(conn: Conn, contact_id: i32) -> anyhow::Result<Contact> {
    Ok(diesel::delete(contacts.filter(id.eq(contact_id))).get_result(&conn)?)
}

pub fn list(conn: Conn, by: Option<ContactQuery>) -> anyhow::Result<Vec<Contact>> {
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
