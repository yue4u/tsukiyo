use super::model::*;
use crate::sql::{
    db::Conn,
    schema::events::{self, dsl::*},
};
use diesel::prelude::*;
use validator::Validate;

pub fn get(conn: Conn, event_id: i32) -> anyhow::Result<Event> {
    Ok(events.filter(id.eq(event_id)).get_result(&conn)?)
}

pub fn create(conn: Conn, event: EventInput) -> anyhow::Result<Event> {
    event.validate()?;
    Ok(diesel::insert_into(events::table)
        .values(&event)
        .get_result::<Event>(&conn)?)
}

pub fn update(conn: Conn, event_id: i32, event: EventUpdate) -> anyhow::Result<Event> {
    Ok(diesel::update(events.filter(id.eq(event_id)))
        .set(&event)
        .get_result(&conn)?)
}

pub fn delete(conn: Conn, event_id: i32) -> anyhow::Result<Event> {
    Ok(diesel::delete(events.filter(id.eq(event_id))).get_result(&conn)?)
}

pub fn list(conn: Conn, by: Option<EventQuery>) -> anyhow::Result<Vec<Event>> {
    let mut query = events::table.into_boxed();
    if let Some(by_event) = by {
        // TODO: simplify this
        if let Some(_published) = by_event.published {
            query = query.filter(events::published.eq(_published));
        }
        if let Some(_genre) = by_event.genre {
            query = query.filter(events::genre.eq(_genre));
        }
        if let Some(_tag) = by_event.tag {
            query = query.filter(events::genre.eq(_tag));
        }
        if let Some(_limit) = by_event.limit {
            query = query.limit(_limit as i64);
        }
    }
    Ok(query.load::<Event>(&conn)?)
}
