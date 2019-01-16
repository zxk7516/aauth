use crate::models::DbExecutor;
use actix::prelude::{Addr, Handler};
use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "星球大战宇宙中的人形生物")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "星球大战宇宙中的人形生物")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}


#[derive(GraphQLObject)]
#[graphql(description = "站点文章")]
struct Post {
    id: i32,
    title: String,
    content: String,
}

pub struct QueryRoot {
    db: Addr<DbExecutor>,
}

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
    field posts(&executor, id: String) -> FieldResult<Human> {
        
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
});

pub struct MutationRoot {
    db: Addr<DbExecutor>,
}

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema(_db: Addr<DbExecutor>) -> Schema {
    Schema::new(
        QueryRoot { db: _db.clone() },
        MutationRoot { db: _db.clone() },
    )
}
