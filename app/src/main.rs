use sea_orm::*;
use std::env;

use ::entity::{users, users::Entity as User};

#[tokio::main]
async fn main() {
    // Connect to database
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    // Insert new row
    let alice = users::ActiveModel {
        name: Set("Alice".to_owned()),
        age: Set(20),
        ..Default::default()
    };
    let alice = alice.insert(&db).await.unwrap();
    println!("insert {:?}", alice);

    // Select alice by primary key
    let alice: Option<users::Model> = User::find_by_id(alice.id).one(&db).await.unwrap();
    println!("select {:?}", alice);

    // Update alice's age
    let mut alice: users::ActiveModel = alice.unwrap().into();
    alice.age = Set(29);
    let alice: users::Model = alice.update(&db).await.unwrap();
    println!("update {:?}", alice);

    // Delete alice
    let res: DeleteResult = alice.delete(&db).await.unwrap();
    println!("delete {:?}", res);
}
