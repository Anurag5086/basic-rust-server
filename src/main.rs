use dotenv;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::{query, query_as, PgPool};
use tide::prelude::*;
use tide::{Body, Request, Response, Server};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Dino {
    id: Uuid,
    name: String,
    weight: i32,
    diet: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let db_pool = make_db_pool().await;
    let mut app = tide::with_state(db_pool);

    app.at("/createdino").post(create_dino);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

pub async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    println!("{}", db_url);
    Pool::new(&db_url).await.unwrap()
}

async fn create_dino(mut req: Request<sqlx::Pool<sqlx::PgConnection>>) -> tide::Result {
    dotenv::dotenv().ok();
    let dino: Dino = req.body_json().await?;
    let db_pool = req.state().clone();

    let row = query_as!(
        Dino,
        r#"
        INSERT INTO dinos (id, name, weight, diet) VALUES
        ($1, $2, $3, $4) returning id, name, weight, diet
        "#,
        dino.id,
        dino.name,
        dino.weight,
        dino.diet
    )
    .fetch_one(&db_pool)
    .await?;

    let mut res = Response::new(201);
    res.set_body(Body::from_json(&row)?);
    Ok(res)
}
