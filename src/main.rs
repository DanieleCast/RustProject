mod models;
extern crate csv;
extern crate postgres;
use postgres::{Client, NoTls};
use csv::ReaderBuilder;
use crate::models::Status;
use actix_web::{web, App, Responder, HttpServer};
use std::io;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
//---------------------------------------------------------------------------------
//FUNCTIONS

//Server Response
async fn status() -> impl Responder{
  web::HttpResponse::Ok().
  json(Status {status: "OK".to_string()}) 
  //"{\"status\": \"UP\"}"
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
  match env::args_os().nth(1) {
      None => Err(From::from("expected 1 argument, but got none")),
      Some(file_path) => Ok(file_path),
  }
}

//Reading the CSV File from the Path.

fn run() -> Result<(), Box<dyn Error>> {
  let file_path = get_first_arg()?;
  let file = File::open(file_path)?;
  let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
  //let mut rdr = csv::Reader::from_reader(file);
  for result in rdr.records() {
      let record = result?;
      println!("{:?}", record);
  }
  Ok(())
}

//----------------------------------------------------------------------------
//                  MAIN ()

#[actix_rt::main]
async fn main() ->io::Result<()>  {

    println!("Servidor iniciado en LocalHost:8080");
    //Conecting to Postgres DB.
    let mut client = Client::connect("host=127.0.0.1 dbname=citizen port=5432 user=postgres password='password'", NoTls).expect("Connection DB Error");
    client.batch_execute("
    CREATE TABLE IF NOT EXISTS PERSONA (
        id              SERIAL PRIMARY KEY,
        ident           INTEGER,
        nombre          VARCHAR,
        genero          VARCHAR,
        e_civil         VARCHAR,
        nacimiento      DATE,
        tel             INTEGER,
        direccion       VARCHAR,
        email           VARCHAR NOT NULL,
        validado        BOOLEAN,
        observacion     VARCHAR
        )
        ").expect("Create Table Error");


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
