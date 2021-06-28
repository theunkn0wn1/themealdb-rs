use crate::api_datamodel::meal::_Meal;

pub struct Meal {
    id: u32,
    name: String,
    drink_alternate: Option<String>,
    category: String,
    instructions: String,
    thumbnail: String,
    tags: Option<Vec<String>>,
    ingreedients: Vec<String>,
    measures: Vec<String>,
    source: String,
    image_source: Option<String>,

    // FIXME: lacking non-null response
    creative_commons_confirmed: Option<bool>,
    // FIXME: lacking non-null response
    date_modified: Option<String>,
}

impl From<crate::api_datamodel::meal::_Meal> for Meal {
    fn from(internal: _Meal) -> Self {
        Self {
            id: internal.idMeal.into(),
            name: internal.strMeal,
            drink_alternate: internal.strDrinkAlternate,
            category: internal.strCategory,
            instructions: internal.strInstructions,
            thumbnail: internal.strMealThumb,
            tags: match internal.strTags {
                None => { None }
                Some(tags) => { tags.split('\n').collect() }
            },
            // This part is ugly but its what the API spits out...
            ingreedients: vec![
                internal.strIngredient1,
                internal.strIngredient2,
                internal.strIngredient3,
                internal.strIngredient4,
                internal.strIngredient5,
                internal.strIngredient6,
                internal.strIngredient7,
                internal.strIngredient8,
                internal.strIngredient9,
                internal.strIngredient10,
                internal.strIngredient11,
                internal.strIngredient12,
                internal.strIngredient13,
                internal.strIngredient14,
                internal.strIngredient15,
                internal.strIngredient16,
                internal.strIngredient17,
                internal.strIngredient18,
                internal.strIngredient19,
                internal.strIngredient20,
            ],
            measures: vec![
                internal.strMeasure1,
                internal.strMeasure2,
                internal.strMeasure3,
                internal.strMeasure4,
                internal.strMeasure5,
                internal.strMeasure6,
                internal.strMeasure7,
                internal.strMeasure8,
                internal.strMeasure9,
                internal.strMeasure10,
                internal.strMeasure11,
                internal.strMeasure12,
                internal.strMeasure13,
                internal.strMeasure14,
                internal.strMeasure15,
                internal.strMeasure16,
                internal.strMeasure17,
                internal.strMeasure18,
                internal.strMeasure19,
                internal.strMeasure20,
            ],
            source: internal.strSource,
            image_source: internal.strImageSource,
            creative_commons_confirmed: internal.strCreativeCommonsConfirmed,
            date_modified: internal.dateModified,
        }
    }
}