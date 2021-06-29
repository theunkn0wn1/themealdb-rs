use serde::{Deserialize, Serialize};
use crate::datamodel::Meal;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct _Meal {
    pub(crate) idMeal: String,
    pub(crate) strMeal: String,
    pub(crate) strCategory: String,
    pub(crate) strArea: String,
    pub(crate) strInstructions: String,
    pub(crate) strMealThumb: String,
    pub(crate) strTags: Option<String>,
    pub(crate) strYoutube: String,
    pub(crate) strIngredient1: Option<String>,
    pub(crate) strIngredient2: Option<String>,
    pub(crate) strIngredient3: Option<String>,
    pub(crate) strIngredient4: Option<String>,
    pub(crate) strIngredient5: Option<String>,
    pub(crate) strIngredient6: Option<String>,
    pub(crate) strIngredient7: Option<String>,
    pub(crate) strIngredient8: Option<String>,
    pub(crate) strIngredient9: Option<String>,
    pub(crate) strIngredient10: Option<String>,
    pub(crate) strIngredient11: Option<String>,
    pub(crate) strIngredient12: Option<String>,
    pub(crate) strIngredient13: Option<String>,
    pub(crate) strIngredient14: Option<String>,
    pub(crate) strIngredient15: Option<String>,
    pub(crate) strIngredient16: Option<String>,
    pub(crate) strIngredient17: Option<String>,
    pub(crate) strIngredient18: Option<String>,
    pub(crate) strIngredient19: Option<String>,
    pub(crate) strIngredient20: Option<String>,
    pub(crate) strMeasure1: Option<String>,
    pub(crate) strMeasure2: Option<String>,
    pub(crate) strMeasure3: Option<String>,
    pub(crate) strMeasure4: Option<String>,
    pub(crate) strMeasure5: Option<String>,
    pub(crate) strMeasure6: Option<String>,
    pub(crate) strMeasure7: Option<String>,
    pub(crate) strMeasure8: Option<String>,
    pub(crate) strMeasure9: Option<String>,
    pub(crate) strMeasure10: Option<String>,
    pub(crate) strMeasure11: Option<String>,
    pub(crate) strMeasure12: Option<String>,
    pub(crate) strMeasure13: Option<String>,
    pub(crate) strMeasure14: Option<String>,
    pub(crate) strMeasure15: Option<String>,
    pub(crate) strMeasure16: Option<String>,
    pub(crate) strMeasure17: Option<String>,
    pub(crate) strMeasure18: Option<String>,
    pub(crate) strMeasure19: Option<String>,
    pub(crate) strMeasure20: Option<String>,
    pub(crate) strSource: Option<String>,
    pub(crate) strImageSource: Option<String>,
    pub(crate) strCreativeCommonsConfirmed: Option<bool>,
    pub(crate) dateModified: Option<String>,
    pub(crate) strDrinkAlternate: Option<String>,
}


impl Into<Meal> for _Meal{
    fn into(self) -> Meal {
        Meal {
            id: self.idMeal.parse().unwrap(),
            name: self.strMeal,
            drink_alternate: self.strDrinkAlternate,
            category: self.strCategory,
            instructions: self.strInstructions,
            thumbnail: self.strMealThumb,
            tags: self.strTags,
            // This part is ugly but its what the API spits out...
            ingreedients: [
                self.strIngredient1,
                self.strIngredient2,
                self.strIngredient3,
                self.strIngredient4,
                self.strIngredient5,
                self.strIngredient6,
                self.strIngredient7,
                self.strIngredient8,
                self.strIngredient9,
                self.strIngredient10,
                self.strIngredient11,
                self.strIngredient12,
                self.strIngredient13,
                self.strIngredient14,
                self.strIngredient15,
                self.strIngredient16,
                self.strIngredient17,
                self.strIngredient18,
                self.strIngredient19,
                self.strIngredient20,
            ]
                .iter()
                // filter null out
                .filter(|mabie_value| mabie_value.is_some())
                // filter empties out
                .filter(|value| value.as_ref().unwrap().ne("".into()))
                .map(|value| value.as_ref().unwrap().to_owned())
                .collect::<Vec<String>>(),

            measures: [
                self.strMeasure1,
                self.strMeasure2,
                self.strMeasure3,
                self.strMeasure4,
                self.strMeasure5,
                self.strMeasure6,
                self.strMeasure7,
                self.strMeasure8,
                self.strMeasure9,
                self.strMeasure10,
                self.strMeasure11,
                self.strMeasure12,
                self.strMeasure13,
                self.strMeasure14,
                self.strMeasure15,
                self.strMeasure16,
                self.strMeasure17,
                self.strMeasure18,
                self.strMeasure19,
                self.strMeasure20,
            ]
                .iter()
                // filter null out
                .filter(|mabie_value| mabie_value.is_some())
                // filter empties out
                .filter(|value| value.as_ref().unwrap().ne("".into()))
                .map(|value| value.as_ref().unwrap().to_owned())
                .collect::<Vec<String>>(),

            source: self.strSource,
            image_source: self.strImageSource,
            creative_commons_confirmed: self.strCreativeCommonsConfirmed,
            date_modified: self.dateModified,
        }

    }
}