
// This struct holds an Animal, and the type of animal is parameterized as generic type T. 
// This means, technically, the Animal struct could take in an animal, OR a non-animal. Like a computer model!

#[derive(Debug)]
pub struct Animal<T> {
    animal: T,
    name: String,
}

#[derive(Debug)]
pub enum Animals {
    Cat,
    Dog,
}

pub fn perry() -> Animal<Animals> {
    Animal {
        animal: Animals::Dog,
        name: "Perry".to_string(),
    }
}

// In this function, we construct an Animal
// Animal<T> where T = ComputerModels<String>

pub fn animal_computer() -> Animal<ComputerModels<String>> {

    Animal {
        animal: ComputerModels::C12400("Model C12, MK Ver. 400, 12/32".to_string()),
        name: "The Computer Animal!".to_string(),
    }

}

// There are different Computer Models, and each of them contains certain data. This data can be of various types.
// For example, maybe the PC 500 has a String inside, while PC 734 has a i32. We should handle these cases using
// a match statement.

#[derive(Debug)]
pub enum ComputerModels<T> {
    C12400(T),
    C50035(T),
    C73412(T),
}

pub fn computer_data() {
    let PC_1 = ComputerModels::C12400("Model C12, MK Ver. 400, 12/32");
    let PC_2 = ComputerModels::C50035(52);
    let PC_3 = ComputerModels::C50035(23422.01);

    match PC_3 {
        ComputerModels::C12400(a) => println!("{}", a),
        ComputerModels::C50035(a) => println!("{}", a),
        ComputerModels::C73412(a) => println!("{}", a),
    }

}