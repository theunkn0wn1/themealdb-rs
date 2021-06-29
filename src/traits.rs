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
