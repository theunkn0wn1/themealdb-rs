use serde::{Deserialize, Serialize};

use crate::api_datamodel::area::_Area;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct _AreaListResponse {
    pub(crate) meals: Vec<_Area>,
}