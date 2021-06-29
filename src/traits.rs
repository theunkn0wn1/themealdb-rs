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

use crate::datamodel::{Category, Ingredient, Meal};
use crate::Result;

use async_trait::async_trait;

#[async_trait]
/// MealDB features available to all users
pub trait MealDbBaseV1 {
    /// Searches for a meal by its specified name
    /// Returns a optional list of resulting meals
    ///
    /// ```no_run
    /// # use mealdb::prelude::*;
    /// # async fn doc_search_meal_by_name() -> mealdb::Result<()>{
    /// #   let api = mealdb::V1::new("...", "...");
    ///     if let Some(matches) = api.search_meal_by_name("chicken").await?{
    ///         println!("Found some matches!");
    ///     }
    /// #   Ok(())
    /// # }
    /// ```
    async fn search_meal_by_name(&self, name: &str) -> Result<Option<Vec<Meal>>>;
    /// Finds all meals that start with the specified letter.
    /// Same as `search_meal_by_name` with a single-character string.
    async fn search_meal_by_first_letter(&self, letter: &char) -> Result<Option<Vec<Meal>>> {
        self.search_meal_by_name(&letter.to_string()).await
    }
    /// Returns the specified meal by its ID, provided it exists.
    async fn get_meal(&self, id: &str) -> Result<Option<Meal>>;
    /// Returns a random meal.
    async fn get_random_meal(&self) -> Result<Meal>;
    /// Returns the names of all categories.
    async fn list_categories(&self) -> Result<Vec<String>>;
    /// Returns the details of all categories.
    async fn get_categories(&self) -> Result<Vec<Category>>;
    /// Returns the names of all areas.
    async fn list_areas(&self) -> Result<Vec<String>>;
    /// returns the details of all ingredients
    async fn list_ingredients(&self) -> Result<Vec<Ingredient>>;

    /// Returns the IDs of all meals containing the specified main ingredient, if any exist.
    async fn filter_by_main_ingredient(&self, ingredient: &str) -> Result<Option<Vec<String>>>;

    /// Returns the IDs of all meals in the specified category, if any exist.
    async fn filter_by_category(&self, category: &str) -> Result<Option<Vec<String>>>;
    /// Returns the IDs of all meals in the specified area, if any exist.
    async fn filter_by_area(&self, category: &str) -> Result<Option<Vec<String>>>;
}

#[async_trait]
/// MealDB features only available to patreon supporters
trait MealDbPatreonV1 {}
