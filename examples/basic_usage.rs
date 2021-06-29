use mealdb::traits::MealDbBaseV1;
use mealdb::V1;

#[tokio::main]
pub async fn main() {
    println!("hello, world!");

    // Create handle to the API
    let api = V1::new("https://www.themealdb.com", "1");
    println!("finding all meals with `chicken` in the name...");
    let findings = api
        .search_meal_by_name("chicken")
        .await
        .expect("query failed.")
        .expect("no results.");
    findings
        .iter()
        .for_each(|meal| println!("meal name :: {} meal id :: {}", meal.name, meal.id));
    println!("finding meal with specific id...");
    let meal = api.get_meal("52875").await.expect("query failed.").expect("API returned null");
    println!("meal name :: {:?}", meal.name);

    println!("finding all categories (names)....");
    let category_names = api.list_categories().await.expect("query failed.");
    println!("{:?}", category_names);

    println!("finding all areas (names)...");

    let area_names = api.list_areas().await.expect("query failed.");
    println!("{:?}", area_names);
}
