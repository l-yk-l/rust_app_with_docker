use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::NaiveDateTime;
use tokio_postgres::error::Error;
use tokio_postgres::NoTls;
use warp::{path::FullPath, reject, Filter};

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

const CONN: &str = "postgresql://yury:1111@postgres:5432/urllog";

#[derive(Debug)]
struct ConnError;

impl reject::Reject for ConnError {}

#[derive(Debug)]
struct DataError;

impl reject::Reject for DataError {}

async fn list_all(pool: ConnectionPool) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = pool.get().await.map_err(|_| reject::custom(ConnError))?;

    let mut res = String::from("");

    for row in conn
        .query("SELECT log_id, log_text, created_at FROM log", &[])
        .await
        .map_err(|_| reject::custom(ConnError))?
    {
        let (log_id, log_text, created_at): (
            Result<i32, Error>,
            Result<String, Error>,
            Result<NaiveDateTime, Error>,
        ) = (row.try_get(0), row.try_get(1), row.try_get(2));

        res.push_str(&format!(
            " | {} | {} | {} |\n",
            log_id.map_err(|_| reject::custom(DataError))?,
            log_text.map_err(|_| reject::custom(DataError))?,
            created_at.map_err(|_| reject::custom(DataError))?
        ));
    }

    Ok(res)
}

async fn insert_row(
    path: FullPath,
    pool: ConnectionPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = pool.get().await.map_err(|_| reject::custom(ConnError))?;

    conn.execute("INSERT INTO log (log_text) VALUES ($1)", &[&path.as_str()])
        .await
        .map_err(|_| reject::custom(ConnError))?;

    return Ok(format!("Log was added. Url:{}\n", path.as_str()));
}

fn with_pool(
    pool: ConnectionPool,
) -> impl Filter<Extract = (ConnectionPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

#[tokio::main]
async fn main() {
    let manager = PostgresConnectionManager::new_from_stringlike(CONN, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let list = warp::path!("list")
        .and(with_pool(pool.clone()))
        .and_then(list_all);

    let insert = warp::path::full().and(with_pool(pool)).and_then(insert_row);

    let routes = list.or(insert);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}