use warp::{Filter, path::FullPath};
use postgres::{Client, NoTls, Error};
use chrono::prelude::*;

const CONN : &str = "postgresql://yury@localhost:5432/urllog"; 


fn list_all() -> Result<String, Error> {
    let client = Client::connect(
        CONN,
        NoTls,
    );
    let mut res = String::from("");
    match client {
        Ok(mut good_client) => 
            for row in good_client.query("SELECT log_id, log_text, created_at FROM log", &[])? {
                let (log_id, log_text, created_at) : (Option<i32>, Option<String>, Option<NaiveDateTime>) 
                    = (row.get (0), row.get (1), row.get(2));
                
                if log_id.is_some () && log_text.is_some () && created_at.is_some() {
                    res.push_str(" | ");
                    res.push_str(log_id.unwrap().to_string().as_str());
                    res.push_str(" | ");
                    res.push_str(log_text.unwrap().as_str());
                    res.push_str(" | ");
                    res.push_str(created_at.unwrap().to_string().as_str());
                    res.push_str(" | \n");
                };
            },
        Err(e) => panic!("{}", e),
    };

    return Ok(res);
}

fn insert_row(path: &FullPath) -> String {
    let client = Client::connect(
        CONN,
        NoTls,
    );
    
    match client {
        Ok(mut good_client) => good_client.execute(
            "INSERT INTO log (log_text) VALUES ($1)",
            &[&path.as_str().to_string()],
        ),
        Err(e) => panic!("{}", e),
    }.ok();

    return format!("Log was added. Url:{}\n", path.as_str().to_string());
}

#[tokio::main]
async fn main() {
    let list = warp::path!("list")
        .map(|| {
            let res = list_all();
            let res2 = match res {
                Ok(ref _r) => res.unwrap(),
                Err(e) => panic!("{}", e),
            };
            
            return res2;
        });

    let insert = warp::path::full()
    .map(move |path: FullPath| insert_row(&path));

    let routes = list.or(insert);


    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
