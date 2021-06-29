# mealdb-rs

This rust crate provides a `reqwest`-based async interface
to [`themealdb.com`](https://www.themealdb.com).

## Usage

Currently, this crate only supports the json API v1 endpoints.

To use this crate, instantiate an instance of the API wrapper.

```rs
use mealdb::preamble::*;

#[tokio::main]
pub async fn main() {
    println!("hello, world!");

    // Create handle to the API
    let api = mealdb::V1::new("https://www.themealdb.com", "<YOUR API KEY HERE>");
    // ...
    }
```

this `V1` api wrapper exposes the supported methods of `themealdb`'s API.

For example, getting all the meals with a title containing `chicken`:

```rs
    // Searching for meals that contain a keyword
    println!("finding all meals with `chicken` in the name...");
    let meals = api
        .search_meal_by_name("chicken")
        .await
        .expect("query failed.")
        .expect("no results.");
    meals
        .iter()
        .for_each(|meal| println!("meal name :: {:?} meal id :: {}", meal.name, meal.id));

```