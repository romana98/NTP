use mongodb::{ sync::Client, sync::Database};

const DB_NAME: &str = "test";

static mut COLLECTION_ADMIN: &str = "admin";
static mut COLLECTION_HC: &str = "hard_constraint";
static mut COLLECTION_SC: &str = "soft_constraint";
static mut COLLECTION_STAFF: &str = "staff";
static mut COLLECTION_LECTURE: &str = "lecture";
static mut COLLECTION_FACULTY: &str = "faculty";
static mut COLLECTION_SCHEDULE: &str = "schedule";
static mut COLLECTION_SHIFT: &str = "shift";


#[derive(Clone, Debug)]
pub struct DB {
    pub db: Database,
}

impl DB {
    pub fn init() -> mongodb::error::Result<Self> {
        // Get a handle to the cluster
        let client = Client::with_uri_str(
            "mongodb://localhost:27017",
        )?;

        /*// Ping the server to see if you can connect to the cluster
        client
            .database("test")
            .run_command(doc! {"ping": 1}, None)?;
        println!("Connected successfully.");

       // List the names of the databases in that cluster
        for db_name in client.list_database_names(None, None)? {
            println!("{}", db_name);
        }

        // Get a handle to a database.
        let db = client.database("test");

        // List the names of the collections in that database.
        for collection_name in db.list_collection_names(None)? {
            println!("{}", collection_name);
        }

        // Get a handle to a collection in the database.
        let collection = db.collection::<Document>("books");

        let docs = vec![
            doc! { "title": "1984", "author": "George Orwell" },
            doc! { "title": "Animal Farm", "author": "George Orwell" },
            doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
        ];

        // Insert some documents into the "test.books" collection.
        collection.insert_many(docs, None)?;

        let cursor = collection.find(doc! { "author": "George Orwell" }, None)?;
        for result in cursor {
            println!("title: {}", result?);
        }*/

        //Ok("Connected to MongoDB")
        Ok(Self {
            db:   client
                .database("test")
        })
    }
}