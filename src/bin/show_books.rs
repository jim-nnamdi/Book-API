extern crate books;
extern crate diesel;

use diesel::prelude::*;
use books::*;
use self::models::*;

fn main(){
  use self::schema::books::dsl::*;

  let connection = establish_connection();

  let results = books.filter(published.eq(true)).limit(5).load::<Book>(&connection).expect("Error in loading books");

  println!("Total books in library are {}", results.len());

  for book in results {
    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Published: {}", book.published);
  }
}