mod models;
mod sql_fun;
use crate::sql_fun::create_table;
use crate::sql_fun::write_in_db;
extern crate csv;
//extern crate postgres;
//use postgres::{Client, NoTls};
use csv::ReaderBuilder;
use crate::models::Status;
use crate::models::Row;
use actix_web::{web, App, Responder, HttpServer};
use std::io;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

//Server Response
async fn status() -> impl Responder{
  web::HttpResponse::Ok().
  json(Status {status: "OK".to_string()}) 
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
  match env::args_os().nth(1) {
      None => Err(From::from("expected 1 argument, but got none")),
      Some(file_path) => Ok(file_path),
  }
}

//Reading the CSV File from the Path and Writing in the PostgreSQL DB.

fn run() -> Result<(), Box<dyn Error>> {
  
  //Conecting to Postgres DB.
  create_table();
  let file_path = get_first_arg()?;
  let file = File::open(file_path)?;
  let mut rdr = ReaderBuilder::new().delimiter(b';').has_headers(false).from_reader(file);
  
  for result in rdr.records() {
      let record = result?;
      let row: Row = record.deserialize(None)?;
      write_in_db(row);
  }
  Ok(())
}
//----------------------------------------------------------------------------
//                  MAIN ()
#[actix_rt::main]
async fn main() ->io::Result<()>  {

    println!("Servidor iniciado en LocalHost:8080");
    if let Err(err) = run() {
      println!("{}", err);
      process::exit(1);
    }
    HttpServer::new(||{
        App::new()
            .route("/",web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
