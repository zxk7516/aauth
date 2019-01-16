use crate::errors::ServiceError;
use crate::models::DbExecutor;
use actix::prelude::{Handler, Message};

#[derive(Debug, Deserialize)]
struct PostQuery {
    id: i32,
}

impl Message for PostQuery {
    type Result = Result<(), ServiceError>;
}

impl Handler<PostQuery> for DbExecutor {
    type Result = Result<(), ServiceError>;
    fn handle(&mut self, query: PostQuery, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}
