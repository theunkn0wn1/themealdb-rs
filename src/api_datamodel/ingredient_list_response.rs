use serde::{Deserialize, Serialize};

use crate::api_datamodel::ingredient::_Ingredient;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct _IngredientListResponse {
    pub(crate) meals: Vec<_Ingredient>,
}
