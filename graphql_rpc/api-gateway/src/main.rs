mod graphql;
mod rpc;
pub mod starwars_capnp {
    include!(concat!(env!("OUT_DIR"), "/starwars_capnp.rs"));
}

use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use graphql::schema::{create_schema, Schema};
use std::io;
use std::sync::Arc;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8000/");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let addr = "127.0.0.1:8000";

    println!("API Gateway: http://{}", addr);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/").guard(guard::Post()).to(graphql))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
    }).bind(addr)?
    .run()
    .await
}
