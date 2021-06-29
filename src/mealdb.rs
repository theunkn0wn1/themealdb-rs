use async_trait::async_trait;
use reqwest::get;
use serde_json::from_str;

use crate::datamodel::{Category, Meal};
use crate::traits::MealDbBaseV1;
use crate::Result;

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

        let data: crate::api_datamodel::meal_list_response::_MealsResponse =
            serde_json::from_str(&response)?;

        if let Some(v) = data.meals {
            Ok(Some(
                v.into_iter()
                    .map(|internal| internal.into())
                    .collect::<Vec<Meal>>(),
            ))
        } else {
            Ok(None)
        }
    }

    async fn get_meal(&self, id: &str) -> Result<Option<Meal>> {
        let url = format!("{}/lookup.php?i={}", self.base_uri, id);
        let response = get(url).await?.text().await?;
        let data: crate::api_datamodel::meal_list_response::_MealsResponse =
            serde_json::from_str(&response)?;

        if let Some(v) = data.meals {
            Ok(v.into_iter().map(|internal| internal.into()).next())
        } else {
            Ok(None)
        }
    }

    async fn get_random_meal(&self) -> Result<Meal> {
        let url = format!("{}/random.php", self.base_uri);
        let response = get(url).await?.text().await?;
        let data: crate::api_datamodel::meal_list_response::_MealsResponse =
            serde_json::from_str(&response)?;

        if let Some(mut v) = data.meals {
            if let Some(internal_meal) = v.pop() {
                return Ok(internal_meal.into());
            }
        }
        Err(crate::Error::InvalidAPIResponse)
    }

    async fn list_categories(&self) -> Result<Vec<String>> {
        let url = format!("{}/list.php?c=list", self.base_uri);
        let response = get(url).await?.text().await?;
        let data: crate::api_datamodel::categories_response::_ListCategoriesVariant1Response =
            serde_json::from_str(&response)?;

        Ok(data
            .meals
            .into_iter()
            .map(|response| response.into())
            .collect::<Vec<String>>())
    }

    async fn get_categories(&self) -> Result<Vec<Category>> {
        let response = get(format!("{}/categories.php", self.base_uri))
            .await?
            .text()
            .await?;
        let data: crate::api_datamodel::categories_response::_ListCategoriesVariant2Response =
            serde_json::from_str(&response)?;

        Ok(data
            .categories
            .into_iter()
            .map(|response| response.into())
            .collect::<Vec<Category>>())
    }

    async fn list_areas(&self) -> Result<Vec<String>> {
        let url = format!("{}/list.php?a=list", self.base_uri);
        let response = get(url).await?.text().await?;
        let data: crate::api_datamodel::area_list_reponse::_AreaListResponse =
            serde_json::from_str(&response)?;

        Ok(data
            .meals
            .into_iter()
            .map(|response| response.into())
            .collect::<Vec<String>>())
    }
}
