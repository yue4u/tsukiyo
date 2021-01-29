use super::model::*;
use crate::Context;
use crate::{
    sql::schema::events::{self, dsl::*},
    utils::OrMessageError,
};
use diesel::prelude::*;
use validator::Validate;

pub fn get(ctx: &Context, query: EventQueryPublic) -> anyhow::Result<Event> {
    if let Some(event_slug) = query.slug {
        let conn = ctx.pool.get()?;
        return Ok(events.filter(slug.eq(event_slug)).first(&conn)?);
    }
    let conn = ctx.pool.get()?;
    Ok(events.find(query.id.or_error("not found")?).first(&conn)?)
}

/// see https://github.com/diesel-rs/diesel/issues/2037
pub fn get_public(ctx: &Context, query_input: EventQueryPublic) -> anyhow::Result<EventPublic> {
    let conn = ctx.pool.get()?;
    let updates = page_view.eq(page_view + 1);
    let returns = (
        id, slug, title, body, genre, tag, fee, ogp_img, start_at, end_at, publish_at, updated_at,
        page_view,
    );
    if let Some(_slug) = query_input.slug {
        return Ok(diesel::update(events.filter(slug.eq(_slug)))
            .set(updates)
            .returning(returns)
            .get_result::<EventPublic>(&conn)?);
    }

    Ok(
        diesel::update(events.filter(id.eq(query_input.id.or_error("not found")?)))
            .set(updates)
            .returning(returns)
            .get_result::<EventPublic>(&conn)?,
    )
}

pub fn create(ctx: &Context, event: EventInput) -> anyhow::Result<Event> {
    event.validate()?;
    let conn = ctx.pool.get()?;
    Ok(diesel::insert_into(events::table)
        .values(&event)
        .get_result::<Event>(&conn)?)
}

pub fn update(ctx: &Context, event_id: i32, mut event: EventUpdate) -> anyhow::Result<Event> {
    event.validate()?;
    event.updated_at = Some(chrono::Utc::now().naive_utc());
    let conn = ctx.pool.get()?;
    Ok(diesel::update(events.filter(id.eq(event_id)))
        .set(&event)
        .get_result(&conn)?)
}

pub fn delete(ctx: &Context, event_id: i32) -> anyhow::Result<Event> {
    let conn = ctx.pool.get()?;
    Ok(diesel::delete(events.filter(id.eq(event_id))).get_result(&conn)?)
}

pub fn list(ctx: &Context, by: Option<EventListQuery>) -> anyhow::Result<Vec<Event>> {
    let conn = ctx.pool.get()?;
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

pub fn list_public(ctx: &Context, by: Option<EventListQuery>) -> anyhow::Result<Vec<EventPublic>> {
    let conn = ctx.pool.get()?;
    let mut query = events::table.into_boxed();
    query = query.filter(events::published.eq(true));
    // TODO: simplify this
    if let Some(by_event) = by {
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
    // see https://github.com/diesel-rs/diesel/issues/2037
    Ok(query
        .select((
            id, slug, title, body, genre, tag, fee, ogp_img, start_at, end_at, publish_at,
            updated_at, page_view,
        ))
        .load::<EventPublic>(&conn)?)
}
