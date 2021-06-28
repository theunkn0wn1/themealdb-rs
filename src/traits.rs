use async_trait::async_trait;

#[async_trait]
/// MealDB features available to all
pub trait MealDbBaseV1 {
    async fn search_meal_by_name(name :&str) -> Vec<Meal>;
    async fn search_meal_by_first_letter(letter: &char);
    async fn get_meal(id: &str);
    async fn get_random_meal();

}

#[async_trait]
/// MealDB features only available to patreon supporters
trait MealDbPatreonV1 {

}