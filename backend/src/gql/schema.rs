use crate::events::{
    self,
    model::{Event, EventInput, EventUpdate},
};
use crate::Context;
use juniper::FieldResult;
use juniper::{graphql_object, EmptySubscription};

pub(crate) struct Query;

#[graphql_object(
    context = Context,
)]
impl Query {
    fn api_version() -> &str {
        "0.1"
    }
    fn event(context: &Context, id: i32) -> FieldResult<Event> {
        // Get a db connection.
        let conn = context.pool.get()?;
        let event = crate::events::service::get(conn, id)?;
        // Return the result.
        Ok(event)
    }

    fn events(context: &Context) -> FieldResult<Vec<Event>> {
        let conn = context.pool.get()?;
        let events = crate::events::service::list(conn)?;
        Ok(events)
    }
}

pub(crate) struct Mutation;

#[graphql_object(
    context = Context,
)]
impl Mutation {
    fn create_event(context: &Context, event: EventInput) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::service::create(conn, event)?;
        Ok(event)
    }
    fn set_event_publish(context: &Context, event_id: i32, published: bool) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::service::update(
            conn,
            event_id,
            EventUpdate {
                published: Some(published),
                ..Default::default()
            },
        )?;
        Ok(event)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
