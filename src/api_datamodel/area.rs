use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct _Area {
    pub(crate) strArea: String,
}