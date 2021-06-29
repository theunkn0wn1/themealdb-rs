// BSD 3-Clause License
//
// Copyright (c) 2021, Joshua Salzedo
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use async_trait::async_trait;
use reqwest::get;

use crate::datamodel::{Category, Ingredient, Meal};
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
        self._get_meals_response(url).await
    }

    async fn get_meal(&self, id: &str) -> Result<Option<Meal>> {
        let url = format!("{}/lookup.php?i={}", self.base_uri, id);
        if let Some(mut meals) = self._get_meals_response(url).await? {
            Ok(meals.pop())
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

    async fn list_ingredients(&self) -> Result<Vec<Ingredient>> {
        let url = format!("{}/list.php?i=list", self.base_uri);
        let response = get(url).await?.text().await?;
        let data: crate::api_datamodel::ingredient_list_response::_IngredientListResponse =
            serde_json::from_str(&response)?;

        Ok(data
            .meals
            .into_iter()
            .map(|response| response.into())
            .collect::<Vec<Ingredient>>())
    }

    async fn filter_by_main_ingredient(&self, ingredient: &str) -> Result<Option<Vec<String>>> {
        let url = format!("{}/filter.php?i={}", self.base_uri, ingredient);
        self._get_filtered_meals_response(url).await
    }

    async fn filter_by_category(&self, category: &str) -> Result<Option<Vec<String>>> {
        let url = format!("{}/filter.php?c={}", self.base_uri, category);
        self._get_filtered_meals_response(url).await
    }

    async fn filter_by_area(&self, area: &str) -> Result<Option<Vec<String>>> {
        let url = format!("{}/filter.php?a={}", self.base_uri, area);
        self._get_filtered_meals_response(url).await
    }
}

impl V1 {
    /// Gets a listof Meals from the API.
    /// This has been extracted since its used in multiple functions.
    async fn _get_meals_response(&self, url: String) -> Result<Option<Vec<Meal>>> {
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

    /// Gets a listof Meals from the API.
    /// This has been extracted since its used in multiple functions.
    /// Returns up to `limit` results (ordered by whatever the API returns)
    async fn _get_filtered_meals_response(&self, url: String) -> Result<Option<Vec<String>>> {
        let response = get(url).await?.text().await?;

        // println!("------------");
        // println!("{}", response);
        // println!("------------");
        let data: crate::api_datamodel::filtered_meal_response::_FilteredMealResponse =
            serde_json::from_str(&response)?;

        // Async closures arn't stable yet (https://github.com/rust-lang/rust/issues/62290)
        // therefore we have to do this the hard way.
        if let Some(filtered_meals) = data.meals {
            Ok(Some(
                filtered_meals
                    .into_iter()
                    .map(|response| response.idMeal)
                    .collect::<Vec<String>>(),
            ))
        } else {
            Ok(None)
        }
    }
}
