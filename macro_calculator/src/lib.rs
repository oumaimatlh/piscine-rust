use json::{JsonValue, object};
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats : f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    //Json value est un type fournie par crate json 
    /*
        Json peut contenir plusieurs types de valeurs , 
    jsonValue est une représentation rust capable de stocker toutes ces valeurs
        let name :jsonValue = "oumaima".into();
        ou  bien directly 
            let user:jsonValue = object !{
                "name"=>"oumaima"} ..ect 

    */

    let mut cals:f64 = 0.0;
    let mut carbs:f64 = 0.0;
    let mut proteins:f64 = 0.0;
    let mut fats:f64 = 0.0;
    for food in foods {
        
        cals += food.calories.1.replace("kcal", "").trim().parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins +=  food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;

    }

    let result:JsonValue = object!{
        "cals" => (cals * 100.0).round() / 100.0, 
        "carbs"=> (carbs * 100.0).round() / 100.0, 
        "proteins"=> (proteins * 100.0).round() / 100.0, 
        "fats"=> (fats * 100.0).round() / 100.0, 
    };
    result
}
