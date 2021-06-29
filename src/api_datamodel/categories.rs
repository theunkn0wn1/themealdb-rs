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

use crate::datamodel::Category;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// ther are two variants to the categories depending on which endpoint is queried.
/// this one relates to the response of /list.php?c=list
pub(crate) struct _ListCategoriesVariant1 {
    pub(crate) strCategory: String,
}

impl Into<String> for _ListCategoriesVariant1 {
    fn into(self) -> String {
        self.strCategory
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// there are two variants to the categories depending on which endpoint is queried.
/// this one relates to the response of /categories.php
pub(crate) struct _ListCategoriesVariant2 {
    pub(crate) idCategory: String,
    pub(crate) strCategory: String,
    pub(crate) strCategoryThumb: String,
    pub(crate) strCategoryDescription: String,
}

impl Into<Category> for _ListCategoriesVariant2 {
    fn into(self) -> Category {
        Category {
            id: self.idCategory,
            name: self.strCategory,
            thumbnail: self.strCategoryThumb,
            description: self.strCategoryDescription,
        }
    }
}
