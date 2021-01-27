use crate::{
    auth::User,
    contacts::{self, Contact, ContactInput, ContactQuery, ContactUpdate},
    events::{self, Event, EventInput, EventListQuery, EventUpdate},
    Context,
};
use events::{EventPublic, EventQueryPublic};
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};

impl juniper::Context for Context {}

const API_VERSION: &str = "0.1";

pub struct QueryAdmin;

#[graphql_object(
    context = Context,
)]
impl QueryAdmin {
    fn api_version() -> &str {
        API_VERSION
    }

    fn me(ctx: &Context) -> FieldResult<Option<User>> {
        Ok(ctx.user.clone())
    }

    fn event(ctx: &Context, query: EventQueryPublic) -> FieldResult<Event> {
        let event = events::service::get(ctx, query)?;
        Ok(event)
    }

    #[cfg(debug_assertions)]
    fn event_public(ctx: &Context, query: EventQueryPublic) -> FieldResult<EventPublic> {
        let event = events::service::get_public(ctx, query)?;
        Ok(event)
    }

    fn events(ctx: &Context, by: Option<EventListQuery>) -> FieldResult<Vec<Event>> {
        let events = events::service::list(ctx, by)?;
        Ok(events)
    }

    fn contact(ctx: &Context, id: i32) -> FieldResult<Contact> {
        let contact = contacts::service::get(ctx, id)?;
        Ok(contact)
    }

    fn contacts(ctx: &Context, by: Option<ContactQuery>) -> FieldResult<Vec<Contact>> {
        let contacts = contacts::service::list(ctx, by)?;
        Ok(contacts)
    }
}

pub struct MutationAdmin;

#[graphql_object(
    context = Context,
)]
impl MutationAdmin {
    fn create_event(ctx: &Context, event: EventInput) -> FieldResult<Event> {
        let event = events::create(ctx, event)?;
        Ok(event)
    }

    fn update_event(ctx: &Context, id: i32, update: EventUpdate) -> FieldResult<Event> {
        let event = events::update(ctx, id, update)?;
        Ok(event)
    }

    fn delete_event(ctx: &Context, id: i32) -> FieldResult<Event> {
        let event = events::delete(ctx, id)?;
        Ok(event)
    }

    fn update_contact(ctx: &Context, id: i32, update: ContactUpdate) -> FieldResult<Contact> {
        let contact = contacts::update(ctx, id, update)?;
        Ok(contact)
    }

    fn delete_contact(ctx: &Context, id: i32) -> FieldResult<Contact> {
        let contact = contacts::delete(ctx, id)?;
        Ok(contact)
    }
}

pub type SchemaAdmin = RootNode<'static, QueryAdmin, MutationAdmin, EmptySubscription<Context>>;

pub struct QueryPublic;

#[graphql_object(
    context = Context,
)]
impl QueryPublic {
    fn api_version() -> &str {
        API_VERSION
    }

    fn event(ctx: &Context, query: EventQueryPublic) -> FieldResult<EventPublic> {
        let event = events::service::get_public(ctx, query)?;
        Ok(event)
    }

    fn events(ctx: &Context, by: Option<EventListQuery>) -> FieldResult<Vec<EventPublic>> {
        let events = events::service::list_public(ctx, by)?;
        Ok(events)
    }
}

pub struct MutationPublic;

#[graphql_object(
    context = Context,
)]
impl MutationPublic {
    fn create_contact(ctx: &Context, contact: ContactInput) -> FieldResult<Contact> {
        let contact = contacts::create(ctx, contact)?;
        Ok(contact)
    }
}

pub type SchemaPublic = RootNode<'static, QueryPublic, MutationPublic, EmptySubscription<Context>>;

pub struct Schema {
    pub admin: SchemaAdmin,
    pub public: SchemaPublic,
}

pub fn create_schema() -> Schema {
    Schema {
        admin: SchemaAdmin::new(QueryAdmin, MutationAdmin, EmptySubscription::new()),
        public: SchemaPublic::new(QueryPublic, MutationPublic, EmptySubscription::new()),
    }
}
