pub struct One {
    first_layer: Option<Two>
}
pub struct Two {
    second_layer: Option<Three>
}
pub struct Three {
    third_layer: Option<Four>
}

pub struct Four {
    fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        Some(self.first_layer?.second_layer?.third_layer?.fourth_layer?)
    }
}
