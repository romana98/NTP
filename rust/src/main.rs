#![allow(dead_code)]
//use warp::{Filter, Rejection};
use mongodb::{bson::doc, bson::Document};
mod enums;
mod data;
mod routing;

fn main() {

      let db = data::database::DB::init();
      let db = db.unwrap();



      let collection =  db.db.collection::<Document>("books");

      let docs = vec![
            doc! { "title": "1984", "author": "George Orwell" },
            doc! { "title": "Animal Farm", "author": "George Orwell" },
            doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
      ];

      // Insert some documents into the "test.books" collection.
      collection.insert_many(docs, None);


      for collection_name in db.db.list_collection_names(None) {
            println!("{:?}", collection_name);
      }


      //warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

