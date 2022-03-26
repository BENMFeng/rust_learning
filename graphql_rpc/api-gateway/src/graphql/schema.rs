use crate::rpc;
use juniper::{FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A human of any type")]
pub struct Human {
    pub id: String,
    pub name: String,
    pub appears_in: Episode,
    pub home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new human of any type")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Episode,
    pub home_planet: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn show_human(id: String) -> FieldResult<Human> {
        rpc::client::show_human(id)
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        rpc::client::create_human(new_human)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
