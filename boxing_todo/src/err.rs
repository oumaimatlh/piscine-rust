
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}
#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}


impl Display for ParseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}


impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => {Option::None}
            ParseErr::Malformed(_) =>{Option::Some(self)}
        }
    }
}
impl Error for ReadErr {
   fn source(&self) -> Option<&(dyn Error + 'static)> {
        Option::Some(&*self.child_err)
   }
}









// Error ; un trait qui definit un comportement commun entre tous les types d'erreurs sans connaitre son type 
    //cad lorsque un erreur implement ce trait , rust va detecter ce erreur comme un erreur d'ecosystem :impl Error for MyError {}

// Box<dyn Error>:
    /*
        Box: est un smart Pointer qui permet de stocker  un valeur  local (qui en general ce stock ds stack) la valeur ds le heap
        Pourquoi on utilise Box :
            gerer les types dont sa taille n'est pas connue a la compilation 
    */


/*
    Traits:
        A la compilation :le compilateur connaît la définition du trait  et ses méthodes.
        Trait Object :
            est une manière de manipuler une instance d'un type concret à travers un trait,
             sans avoir besoin de connaître son type concret.
        la ou on est besoin dyn 
        
        let instance_dun_type = dyn name_trait ; cad chaque instance de d'un type a un acces a les methodes du trait ou bien implementation 
        si on fait &dyn : reference vers cette instance 
            let dog = Dog;
            let animal: &dyn Animal = &dog; //animal est une variable qui reference vers un trait object Animal

        Rq:
            un trait object se stocke ds mémoire 
            ds l'exemple ci dessous , animal un pointeur qui pointe directement ds tablev qui presente les méthodes du struct ou enum ect 
            mais c'est obligatoire on met &dyn Animal si on enleve & le compiler ne sait pas son size du type et non ces données  ce qui entraine d'utiliser
            Box<dyn Animal> pour ce stocke directement en heap 
                Box<dyn Animal> a une taille connue à la compilation.

                Sur une architecture 64 bits :
                - Box<Dog> : généralement 8 octets (un pointeur).
                - Box<dyn Animal> : généralement 16 octets (un fat pointer).
                Pourquoi 16 octets pour Box<dyn Animal> ?

                Il contient deux pointeurs :
                1. Data pointer : adresse de l'instance concrète (Dog) dans le Heap.
                2. Vtable pointer : adresse de la table des méthodes de Dog.
*/