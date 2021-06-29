use serde::{Deserialize, Serialize};
use crate::datamodel::Category;

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