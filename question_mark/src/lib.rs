pub struct One {
   pub first_layer: Option<Two>,
}
pub struct Two {
    pub second_layer: Option<Three>,
}
pub struct Three {
    pub third_layer: Option<Four>,
}
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        let two = self.first_layer;
        match two {
            Some(v) => {
                let three = v.second_layer;
                match three {
                    Some(v) => {
                        let four = v.third_layer;
                        match four {
                            Some(v)=>{
                                let  result = v.fourth_layer;
                                match result {
                                    Some(r) => {
                                        Option::Some(r)
                                    }
                                    None => {Option::None}

                                }
                            }
                            None => {Option::None}

                        }
                    }
                    None => {Option::None}

                }
            }
            None => {Option::None}
        
        }
    }
}
