use crate::datamodel::Meal;
use async_trait::async_trait;

#[async_trait]
/// MealDB features available to all
pub trait MealDbBaseV1 {
    /// Searches for a meal by its specified name
    /// Returns a optional list of resulting meals
    async fn search_meal_by_name(&self, name: &str) -> crate::Result<Option<Vec<Meal>>>;
    async fn search_meal_by_first_letter(&self, letter: &char);
    async fn get_meal(&self, id: &str);
    async fn get_random_meal(&self);
}

#[async_trait]
/// MealDB features only available to patreon supporters
trait MealDbPatreonV1 {}
