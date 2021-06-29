use serde::{Deserialize, Serialize};

use crate::api_datamodel::categories::{_ListCategoriesVariant1, _ListCategoriesVariant2};

#[derive(Serialize, Deserialize, Debug)]
/// ther are two variants to the categories depending on which endpoint is queried.
/// this one relates to the response of /list.php?c=list
pub(crate) struct _ListCategoriesVariant1Response {
    pub(crate) meals: Vec<_ListCategoriesVariant1>,
}

#[derive(Serialize, Deserialize, Debug)]
/// there are two variants to the categories depending on which endpoint is queried.
/// this one relates to the response of /categories.php
pub(crate) struct _ListCategoriesVariant2Response {
    pub(crate) meals: Vec<_ListCategoriesVariant2>,
}