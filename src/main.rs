
#[derive(Debug, Clone)]
pub struct  LinkedList<T> {
    data: T, 
    next: Option<Box<LinkedList<T>>>, // takes only the size of a data pointer
}

impl <T:std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n:T) {
        self.data += n;
    }
}


fn main() {
    let mut ll = LinkedList{data: 3, next:Some(Box::new(LinkedList { data: 2, next: None } ))}; // 


    if let Some(ref mut v) = ll.next{
        v.add_up(10)
    }

    println!("Hello World, {:?}  ", ll );

    let mut v:Vec<String>  = Vec::new(); // vec has a lenght, a capacity, and pointer to the data

    v.push("Hello".to_string());

    v.push("GoodBye".to_string());

    for i in 1..105 {
        v.push(i.to_string())
    }

    println!("v len= {}", v.len());

    println!("Capacity = {}", v.capacity());

    println!("Vector value = {:?}", v.join(" "));

}   

