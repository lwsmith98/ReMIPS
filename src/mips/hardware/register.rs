struct Register {
    name: String,
    number: i32,
    value: i32,
    reset_value: i32,
}

fn make_register(reg_name: &String, reg_id: i32, reg_val: i32, reset_val: i32) -> Register {
    Register {
        name: reg_name,
        number: reg_id,
        value: reg_val,
        reset_value: reset_val,
    }
}

impl Register {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> i32 {
        self.number
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}
