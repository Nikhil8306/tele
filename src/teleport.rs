use crate::db::{self, Type};

pub fn main () {
    let db = db::DB::new("test", Type::File);

    if db.is_err() {
        println!("Error = {:?}", db.unwrap_err());
    }
}