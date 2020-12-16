

pub fn simulate_bar(num_patrons: u32) {

     // Sally and Xor go to THE BAR together

     let sally = Customer::new("Sally");
     let xor = Customer::new("Xor");
     
     // Sally purchases a coolatta and Xor purchases a Buzzy
 
     let sallys_order = DrinkOrder::new(sally, Drinks::Coolatta);
     let xors_order = DrinkOrder::new(xor, Drinks::Buzzy);
 
     // They find out how much each costs
 
     sallys_order.print();
     xors_order.print();
 
     // They pay together
 
     let mut double_order = GroupOrder::new(vec![sallys_order, xors_order]);
 
     double_order.print_total();

}



struct GroupOrder {
    orders: Vec<DrinkOrder>
}

impl GroupOrder {

    fn new(orders: Vec<DrinkOrder>) -> Self {
        Self {
            orders
        }
    }

    fn print_total(&self) {
        let mut sum : f32 = 0.0;
        for order in &self.orders {
            sum += order.price();
        }
        println!("The group order costs ${}", sum);
    }

}

struct Customer {
    name: String,
}

impl Customer {

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

}

enum Drinks {
    Buzzy,
    Coolatta,
    Tornado,
    Icey,
}

impl Drinks {

    fn name(&self) -> &str {
        match self {
            Drinks::Buzzy => "Buzzy",
            Drinks::Coolatta => "Coolatta",
            Drinks::Tornado => "Tornado",
            Drinks::Icey => "Icey",
        }
    }
    
    fn price(&self) -> f32 {
        match self {
            Drinks::Buzzy => 2.99,
            Drinks::Coolatta => 6.99,
            Drinks::Tornado => 11.99,
            Drinks::Icey => 5.99,
        }
    }

}

struct DrinkOrder {
    drink: Drinks,
    customer: Customer,
}

impl DrinkOrder {

    fn new(customer: Customer, drink: Drinks) -> DrinkOrder {

        DrinkOrder {
            customer,
            drink,
        }

    }

    fn price(&self) -> f32 {
        self.drink.price()
    }

    fn print(&self) -> () {
        println!("{}'s {} costs ${}.", self.customer.name, self.drink.name(), self.drink.price());
    }

}