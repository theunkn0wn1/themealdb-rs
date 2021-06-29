use crate::datamodel::{Category, Meal};

use async_trait::async_trait;

#[async_trait]
/// MealDB features available to all
pub trait MealDbBaseV1 {
    /// Searches for a meal by its specified name
    /// Returns a optional list of resulting meals
    async fn search_meal_by_name(&self, name: &str) -> crate::Result<Option<Vec<Meal>>>;
    /// Finds all meals that start with the specified letter.
    /// Same as `search_meal_by_name` with a single-character string.
    async fn search_meal_by_first_letter(&self, letter: &char) -> crate::Result<Option<Vec<Meal>>> {
        self.search_meal_by_name(&letter.to_string()).await
    }
    async fn get_meal(&self, id: &str) -> crate::Result<Option<Meal>>;
    async fn get_random_meal(&self) -> crate::Result<Meal>;
    async fn list_categories(&self) -> crate::Result<Vec<String>>;
    async fn get_categories(&self) -> crate::Result<Vec<Category>>;
    async fn list_areas(&self) -> crate::Result<Vec<String>>;
    // async fn list_ingreedients(&self);
}

#[async_trait]
/// MealDB features only available to patreon supporters
trait MealDbPatreonV1 {}
