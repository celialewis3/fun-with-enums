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
            interests: vec![Interests::Food],
        }

    }
    
    fn rand_name() -> String {
        let mut rng = rand::thread_rng();
        let n: u64 = rng.gen_range(0, 3);

        if n < 1 {
            "Oliver".to_string()
        } else if n < 2 {
            "Sandra".to_string()
        } else {
            "Jodie".to_string()
        }
    }

    fn rand_interests() -> String {
        let mut rng = rand::thread_rng();
        let n: u64 = rng.gen_range(0, 3);

        if n < 1 {
            "Oliver".to_string()
        } else if n < 2 {
            "Sandra".to_string()
        } else {
            "Carolyn".to_string()
        }
    }

    pub fn greeting(&self) -> () {
        println!("My name is {} and my interests are {:?}", self.name, self.interests);
    }


}


