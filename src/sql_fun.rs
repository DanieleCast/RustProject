use crate::models::Row;
extern crate postgres;
use postgres::{Client, NoTls};



pub fn create_table(){
  
  let mut client = Client::connect("host=127.0.0.1 dbname=citizen port=5432 user=postgres password='password'", NoTls)    
  .expect("Connection DB Error"); 
    client.batch_execute("
      CREATE TABLE IF NOT EXISTS PERSONA (
          id              SERIAL PRIMARY KEY,
          ident           VARCHAR,
          nombre          VARCHAR,
          genero          VARCHAR,
          e_civil         VARCHAR,
          nacimiento      VARCHAR,
          tel             VARCHAR,
          direccion       VARCHAR,
          email           VARCHAR NOT NULL,
          validado        BOOLEAN,
          observacion     VARCHAR
          )
          ").expect("Create Table Error");
}
pub fn write_in_db(row: Row){
  let mut client = Client::connect("host=127.0.0.1 dbname=citizen port=5432 user=postgres password='password'", NoTls)    
  .expect("Connection DB Error"); 
  client.execute(
    "INSERT INTO PERSONA (ident, nombre, genero,e_civil, nacimiento,tel,direccion,email) 
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    &[&row.iden,&row.name.to_uppercase(),&row.gender,&row.civile,&row.birth,&row.phone,&row.dirr,&row.mail],
  ).expect("Insert DB Error"); 
}