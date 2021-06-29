use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// ther are two variants to the categories depending on which endpoint is queried.
/// this one relates to the response of /list.php?c=list
pub(crate) struct _ListCategoriesVariant1 {
    pub(crate) strArea: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// there are two variants to the categories depending on which endpoint is queried.
/// this one relates to the response of /categories.php
pub(crate) struct _ListCategoriesVariant2 {
    idCategory: u32,
    strCategory: String,
    srCategoryThumb: String,
    strCategoryDescription: String,
}