#![feature(proc_macro_hygiene, decl_macro)]

use futures::{FutureExt, TryStreamExt};
use tokio_postgres::{Error, NoTls, Client};

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::postgres;

#[database("my_database")]
struct MyDbConn(postgres::Connection);

#[get("/hello/<name>/<age>")]
async fn hello(conn: MyDbConn, name: String, age: u8) -> Result<String, Error> {
    for row in &conn.query("SELECT $1::TEXT", &[&"hello world"]).unwrap() {
        let x: String = row.get(0);

        println!("Found person {}", x);
    }

    // ^^ this works, but not async
    // vv this does not work

    /*
    let rows = &conn
        .query("SELECT $1::TEXT", &[&"hello world"])
        .unwrap()
        .try_collect::<Vec<_>>()
        .await?;
    */

    Ok(format!("Hello, {} year old named {}!", age, name))
}

fn main() {
    rocket::ignite().attach(MyDbConn::fairing()).mount("/", routes![hello]).launch();
}