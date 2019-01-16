use actix::prelude::{Actor, Handler, Message, SyncContext};

use crate::app::AppState as AppGraphState;
use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse, Json, State};
use futures::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::graph::schema::Schema;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
        type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
        schema: std::sync::Arc<Schema>,
}
impl GraphQLExecutor {
        pub fn new(schema: std::sync::Arc<Schema>) -> GraphQLExecutor {
                GraphQLExecutor { schema: schema }
        }
}

impl Actor for GraphQLExecutor {
        type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
        type Result = Result<String, Error>;

        fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
                let res = msg.0.execute(&self.schema, &());
                let res_text = serde_json::to_string(&res)?;
                Ok(res_text)
        }
}
pub fn graphql(
        (st, data): (State<AppGraphState>, Json<GraphQLData>),
) -> FutureResponse<HttpResponse> {
        st.executor
                .send(data.0)
                .from_err()
                .and_then(|res| match res {
                        Ok(user) => Ok(HttpResponse::Ok()
                                .content_type("application/json")
                                .body(user)),
                        Err(_) => Ok(HttpResponse::InternalServerError().into()),
                })
                .responder()
}
pub fn graphiql(_req: &HttpRequest<AppGraphState>) -> Result<HttpResponse, Error> {
        let html = graphiql_source("http://127.0.0.1:3000/graphql");
        Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
}
