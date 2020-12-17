use rand::Rng;

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    interests: Vec<Interests>,
}

#[derive(Debug, Clone)]
enum Interests {
    Food,
    Politics,
    Videogames,
    Fishing,
    Technology,
    Television,
}

#[derive(Debug)]
enum Topics {
    Food,
    Politics,
    Videogames,
    Television,
}

impl Topics {

    fn rand_topic() -> Topics {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0,3) {
            0 => Topics::Food,
            1 => Topics::Politics,
            2 => Topics::Videogames,
            _ => Topics::Television,
        }
    }
}

pub struct Room {
    people: Vec<Person>,
    //conversations: Vec<Conversation>,
}

impl Room {

    pub fn new(num: u32) -> Self {

        Self {
            people: Room::spawn_people(num)
        }

    }

    pub fn spawn_people(num: u32) -> Vec<Person> {
        let mut cur = 0;
        let mut people: Vec<Person> = Vec::new();

        // if num is greater than available names, num = names.max
        // num must be an even number, unless someone can be talking to themselves?
        // someone can def be talking to themself

        while cur < num {
            people.push(Person::new());
            cur += 1;
        }

        people
    }

    pub fn spawn_conversations(&self) -> Vec<Conversation> {
        let mut iter = self.people.iter();
        let mut conversations: Vec<Conversation> = Vec::new();

        while let Some(person) = iter.next() {

            if let Some(next_person) = iter.next() {
                let conversation = Conversation {
                    people: vec![person.clone(), next_person.clone()],
                    topic: Topics::rand_topic()
                };
                conversations.push(conversation);
            } else {
                let conversation = Conversation {
                    people: vec![person.clone()],
                    topic: Topics::rand_topic()
                };
                conversations.push(conversation);
            }
        }

        conversations
    }

    pub fn talk(&self) {
        let mut count: u32 = 0;

        println!("There are {} people in the room.", self.people.len());
        println!("");

        for convo in Room::spawn_conversations(&self) {
            println!("Conversation {}", count);
            count +=1;

            for person in convo.people {
                println!("{} who likes {:?}", person.name, person.interests)
            }

            println!("The conversation topic is {:?}", convo.topic);
            println!("");


        }
    }

}


pub struct Conversation {
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

        match rng.gen_range(0,10) {
            0 => "Luisa".to_string(),
            1 => "Janelle".to_string(),
            2 => "Celia".to_string(),
            3 => "Rebecca".to_string(),
            4 => "Palak".to_string(),
            5 => "Kelsey".to_string(),
            6 => "Grace".to_string(),
            7 => "Jasmine".to_string(),
            8 => "Karl".to_string(),
            9 => "Mina".to_string(),
            _ => "Juan".to_string(),
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


