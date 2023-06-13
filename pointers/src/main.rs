use std::rc::Rc;
fn main() {
    println!("Hello, world!");
    multiply();
    q2()
}

fn multiply() {
    let x = 5;
    let y = Box::new(5); //heap
    println! {"{}", x * *y};
}

fn q2() {
    let s = String::from("hello");
    let rc: Rc<String> = Rc::new(s);

    println!("{}", Rc::strong_count(&rc));

    {
        let rc2: Rc<String> = Rc::clone(&rc);

        println!("{}", Rc::strong_count(&rc));
        println!("{}", Rc::strong_count(&rc2));
    }

    println!("{}", Rc::strong_count(&rc));
}
