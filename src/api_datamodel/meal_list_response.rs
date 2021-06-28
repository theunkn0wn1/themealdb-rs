use serde::{Serialize, Deserialize};

use crate::api_datamodel::meal::_Meal;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MealsResponse {
    pub(crate) meals: Option<Vec<_Meal>>,
}