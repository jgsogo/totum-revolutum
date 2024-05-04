use self::models::*;
use diesel::prelude::*;
use photodb::*;

fn main() {
    use self::schema::photos::dsl::*;

    let connection = &mut db::establish_connection();
    let results = photos
        .limit(5)
        .select(Photo::as_select())
        .load(connection)
        .expect("Error loading photos");

    println!("Displaying {} photos", results.len());
    for post in results {
        println!("{}", post.id);
        println!("-----------\n");
        println!("{}", post.storage_path);
    }
}
