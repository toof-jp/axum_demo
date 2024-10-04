use std::{fmt, str::FromStr};
use axum::{extract::Query, routing::get, Json, Router};
use serde::{de, Deserialize, Deserializer, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct T {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    // #[serde_as(as = "NoneAsEmptyString")]
    // #[serde(with = "serde_with::rust::string_empty_as_none")]
    a: Option<i32>,
    b: Option<i32>,
}

#[derive(Serialize)]
struct Sum {
    num: i32,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

async fn root_handler(Query(query): Query<T>) -> Json<Sum> {
    let sum = Sum{ num: query.a.unwrap() + query.b.unwrap() };
    Json(sum)
}