use serde::{Deserialize, Serialize};

use crate::api_datamodel::meal::_Meal;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct _MealsResponse {
    pub(crate) meals: Option<Vec<_Meal>>,
}
