#[derive(Debug, Clone)]
pub struct Person {
    name: String, 
    age: i32, 
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age}
    }

    pub fn greet(&self )-> String {
        format!("Hi, My name is {} and my age is {}", self.name, self.age)
    }

    pub fn age_up(&mut self, n:i32) {
        self.age += n
    } 

    pub fn drop_me(self) {}
}

fn main() {

    let mut p = Person::new("Pratik".to_string(), 22);

    p.age_up(3);


    let s  = p.greet();

    println!("Hello World {} ", s);

    //p.drop_me();


    let s2 = p.greet();

    println!("Hello World {} ", s2);
}

pub fn get_age(s:&Person) -> &i32 {
    &s.age
}
