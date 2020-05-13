fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3];

    let mut w = Vec::new();
    w.push(5);
    w.push(6);
    w.push(7);
    w.push(8);

    let x = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match x.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &x[100];
    // unsafe {
    //     let does_not_exist = x.get_unchecked(100);
    //     println!("does_not_exist is {:?}", does_not_exist);
    // }

    let mut y = vec![1, 2, 3, 4, 5];
    let first = &y[0];
    // y.push(6);
    println!("The first element is: {}", first);
}
