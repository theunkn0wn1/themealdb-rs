use async_trait::async_trait;

use crate::api_datamodel::meal::_Meal;
use crate::datamodel::Meal;
use crate::traits::MealDbBaseV1;
use crate::Result;
use reqwest::get;
use serde_json::from_str;

pub struct V1 {
    base_uri: String,
}

impl V1 {
    pub fn new(base_uri: &str, authorization_token: &str) -> Self {
        Self {
            base_uri: format!("{}/api/json/v1/{}", base_uri, authorization_token),
        }
    }
}

#[async_trait]
impl MealDbBaseV1 for V1 {
    async fn search_meal_by_name(&self, name: &str) -> crate::Result<Option<Vec<Meal>>> {
        let url = format!("{}/search.php?s={}", self.base_uri, name);
        let response = get(url).await?.text().await?;

        let data: crate::api_datamodel::meal_list_response::MealsResponse =
            serde_json::from_str(&response)?;

        if let Some(v) = data.meals {
            Ok(Some(
                v.into_iter()
                    .map(|internal| Meal::from(internal))
                    .collect::<Vec<Meal>>(),
            ))
        } else {
            Ok(None)
        }
    }

    async fn search_meal_by_first_letter(&self, letter: &char) {
        todo!()
    }

    async fn get_meal(&self, id: &str) {
        todo!()
    }

    async fn get_random_meal(&self) {
        todo!()
    }
}
