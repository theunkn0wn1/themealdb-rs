use mealdb::prelude::*;

#[tokio::main]
pub async fn main() {
    println!("hello, world!");

    // Create handle to the API
    let api = mealdb::V1::new("https://www.themealdb.com", "1");

    // Searching for meals that contain a keyword
    println!("finding all meals with `chicken` as an ingredient...");
    let meals = api
        .filter_by_main_ingredient("chicken")
        .await
        .expect("query failed.")
        .expect("no results.");
    println!("{:?}", meals);

    // println!("finding meal with specific id...");
    // let meal = api
    //     .get_meal("52875")
    //     .await
    //     .expect("query failed.")
    //     .expect("API returned null");
    // println!("meal name :: {:?} meal id :: {}", meal.name, meal.id);
    //
    // println!("finding a random meal...");
    // let meal = api.get_random_meal().await.expect("query failed");
    // println!("meal name :: {:?} meal id :: {}", meal.name, meal.id);
    //
    println!("list all categories (names)....");
    let category_names = api.list_categories().await.expect("query failed.");
    println!("{:?}", category_names);

    println!("listing all areas (names)...");
    let area_names = api.list_areas().await.expect("query failed.");
    println!("{:?}", area_names);

    // println!("Getting all the categories...");
    // let categories = api.get_categories().await.expect("query failed!");
    // categories.iter().for_each(|cat| {
    //     println!("category name :: {:?} category id :: {}", cat.name, cat.id);
    // });

    // println!("Getting all ingredients...");
    // let ingredients = api.list_ingreedients().await.expect("query failed!");
    // for x in ingredients {
    //     println!(
    //         "name :: {:?}\tid:: {}\ttype ::{:?}",
    //         x.name, x.id, x.ingredient_type
    //     );
    // }

    println!("finding all meals in the `Side` category..");
    let meals = api
        .filter_by_category("Side")
        .await
        .expect("query failed.")
        .expect("no results.");
    println!("{:?}", meals);

    println!("finding all meals in the `British` area..");
    let meals = api
        .filter_by_area("British")
        .await
        .expect("query failed.")
        .expect("no results.");
    println!("{:?}", meals);
}
