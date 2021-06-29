use crate::api_datamodel::categories::_ListCategoriesVariant2;


#[derive(Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub description: String,
}
