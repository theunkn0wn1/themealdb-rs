use mealdb::V1;
use mealdb::traits::MealDbBaseV1;

#[tokio::main]
pub async fn main() {
    println!("hello, world!");

    // Create handle to the API
    let api = V1::new("https://www.themealdb.com", "1");

    let findings = api.search_meal_by_name("chicken").await
        .expect("query failed.").expect("no results.");
    println!("Found something! {:?}", findings)
}