// use chrono::prelude::*;
use mysql::prelude::Queryable;
use std::env;
use chrono::Utc;
use mysql::params;

fn main() {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // create timestamp table if it doesn't exist
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS timestamp (
        id int NOT NULL AUTO_INCREMENT PRIMARY KEY,
        last_updated timestamp NOT NULL
      );",
    )
    .expect("could not create the table `timestamp`");

    println!("`timestamp` table created or already existed");

    // fetch current timestamp
    let timestamp: Option<mysql::Row> = conn
        .query_first("SELECT id, last_updated FROM timestamp")
        .expect("could not fetch the timestamp record");

    match timestamp {
        Some(timestamp_row) => {
            println!("current timestamp: {:?}. updating...", timestamp_row);
        }
        None => {
            println!("no timestamp exists. creating...");
            let current_timestamp = Utc::now().to_string();

            conn.query_drop(
                format!("INSERT INTO timestamp (last_updated) VALUES('{}')", current_timestamp),
            )
            .expect("could not insert timestamp value");
        }
    }

    println!("DONE")
}
