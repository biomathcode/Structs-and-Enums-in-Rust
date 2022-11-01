#[derive(Debug)]
pub struct Person {
    name: String,
    age:i32, 
    children: i32, 
    fave_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Red(String), 
    Green, 
    Blue, 
}

impl Person {
    pub fn print(self)-> String{
        format!("name = {}, age= {}, children = {}", self.name, self.age, self.children)
    }
}


fn main() {
    let p = Person {
        name: "Pratik".to_string(), 
        age:35 , 
        children: 4, 
        fave_color: Color::Green,
    };

    let c = Color::Red("hello".to_string());

    match c {
        Color::Red(s) => println!("It's Red {}", s),
        Color::Blue => println!("It's Blue"),
        Color::Green => println!("It's Green"),
    }


    println!("Person details are {:?}", p);
}
