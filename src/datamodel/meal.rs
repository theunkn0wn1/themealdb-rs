
#[derive(Debug)]
pub struct Meal {
    pub id: String,
    pub name: String,
    pub drink_alternate: Option<String>,
    pub category: String,
    pub instructions: String,
    pub thumbnail: String,
    pub tags: Option<String>,
    pub ingreedients: Vec<String>,
    pub measures: Vec<String>,
    pub source: Option<String>,
    pub image_source: Option<String>,

    // FIXME: lacking non-null response
    pub creative_commons_confirmed: Option<bool>,
    // FIXME: lacking non-null response
    pub date_modified: Option<String>,
}
