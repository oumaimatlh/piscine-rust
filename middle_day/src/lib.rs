use chrono::{NaiveDate, Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {

    let date = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;

    if date.leap_year() {
        return None;
    }

    let middle_day = 183;

    let middle_day_year = NaiveDate::from_yo_opt(year as i32, middle_day)?;

    Some(middle_day_year.weekday())
}

/*
    crate : unite de compilation ;
        -rust considere main.rs comme la racine d'un crate
        -ensemble du code que rust compile ensemble enc commencant oar un fichier racine
        TYpes crates :
            Binary crate : Executer un programme => executable
                        -C'est un crate dont l'objectif est de produire un programme que tu peux exécuter.
            Library crate : Fournir du code réutisable => library
  
    src/main.rs — racine du crate
        déclare les modules
        Module math
        src/math.rs

        Module utils
        src/utils.rs

Les modules sont organisés séparément, mais tous font partie du même crate compilé.
Depuis la racine du crate actuel

Package = la boîte qui contient ton projet.
Crate = un programme ou une bibliothèque compilée dans cette boîte.
Module = les différentes parties organisées à l'intérieur de chaque crate.
Cargo = l'outil qui gère la boîte, les dépendances et la compilation.


Chrono :
est une crate (bib externe) qui permet de travailler avec dates &&heures
Avec chrono, tu peux :
- Récupérer la date et l'heure actuelles.
- Formater une date sous forme de String.
- Parser une chaîne de caractères en date.
- Manipuler les dates et les heures.
- Gérer les fuseaux horaires (UTC, heure locale, etc.).
- Comparer deux dates. 
*/
