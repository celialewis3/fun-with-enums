use rand::Rng;


pub struct Person {
    name: String,
    interests: Vec<Interests>,
}

#[derive(Debug)]
enum Interests {
    Food,
    Politics,
    Videogames,
    Fishing,
    Technology,
    Television,
}

enum Topics {
    Food,
    Politics,
    Videogames,
    Television,
}

struct Group {
    people: Vec<Person>,
}

struct Room {
    groups: Vec<Group>,
}

struct Conversation {
    people: Vec<Person>,
    topic: Topics,
}

impl Person {

    pub fn new() -> Self {

        Self {
            name: Person::rand_name(),
            interests: Person::rand_interest_vec(),
        }

    }
    
    fn rand_name() -> String {
        let mut rng = rand::thread_rng();
        let n: u64 = rng.gen_range(0, 3);

        if n < 1 {
            "Oliver".to_string()
        } else if n < 2 {
            "Carolyn".to_string()
        } else {
            "Luisa".to_string()
        }
    }

    fn rand_interest() -> Interests {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0,6) {
            0 => Interests::Fishing,
            1 => Interests::Food,
            2 => Interests::Politics,
            3 => Interests::Technology,
            4 => Interests::Television,
            _ => Interests::Videogames,
        }
    }

    fn rand_interest_vec() -> Vec<Interests> {
        vec![Person::rand_interest(), Person::rand_interest()]
    }

    pub fn greeting(&self) -> () {
        println!("My name is {} and my interests are {:?}", self.name, self.interests);
    }


}


