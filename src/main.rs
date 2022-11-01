pub struct  Stepper {
    curr:i32, 
    step:i32, 
    max: i32,
}

impl Iterator for Stepper {
    type Item= i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut n = 0;

    let mut st = Stepper{curr: 2, step: 3, max: 15};

    loop {
        match st.next() {
            Some(v) => println!("Something {}", v),
            None => break,
        }
    }
    loop {
        n += 1;
        if  n > 10 {
            break;
        }
        println!("hello {}", n);
    }
    println!("All is Done");

    // while n < 20 {
    //     println!("hello, {}!", n);

    //     n += 1;
    // }

    // for i in 1..10 {
    //     println!("HI {}", i);
    // }
}
