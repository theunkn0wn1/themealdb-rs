use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct _Ingredient {
    pub(crate) idIngredient: u64,
    pub(crate) strIngredient: String,
    pub(crate) strDescription: String,
    pub(crate) strType: Option<String>,
}
