use chrono::prelude::*;
use postgres::{Client, Error, NoTls};
use warp::{path::FullPath, Filter};

const CONN: &str = "postgresql://yury:1111@postgres:5432/urllog";

fn list_all() -> Result<String, Error> {
    let client = Client::connect(CONN, NoTls);

    let mut res = String::from("");

    match client {
        Ok(mut good_client) => {
            for row in good_client.query("SELECT log_id, log_text, created_at FROM log", &[])? {
                let (log_id, log_text, created_at): (
                    Option<i32>,
                    Option<String>,
                    Option<NaiveDateTime>,
                ) = (row.get(0), row.get(1), row.get(2));

                if log_id.is_some() && log_text.is_some() && created_at.is_some() {
                    res.push_str(&format!(
                        " | {} | {} | {} |\n",
                        log_id.unwrap(),
                        log_text.unwrap(),
                        created_at.unwrap()
                    ));
                };
            }
        }
        Err(e) => panic!("LIST: {}", e),
    };

    Ok(res)
}

fn insert_row(path: &FullPath) -> String {
    let client = Client::connect(CONN, NoTls);

    match client {
        Ok(mut good_client) => good_client.execute(
            "INSERT INTO log (log_text) VALUES ($1)",
            &[&path.as_str().to_string()],
        ),
        Err(e) => panic!("INSERT {}", e),
    }
    .ok();

    return format!("Log was added. Url:{}\n", path.as_str());
}

#[tokio::main]
async fn main() {
    let list = warp::path!("list").map(|| match list_all() {
        Ok(list) => list,
        Err(e) => panic!("{}", e),
    });

    let insert = warp::path::full().map(move |path: FullPath| insert_row(&path));

    let routes = list.or(insert);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
