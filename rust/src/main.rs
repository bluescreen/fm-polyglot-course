enum RSEnum {
    Foo(i32),
    Bar(String),
    Baz(Vec<String>)
}

fn main() {
    let mut a:Vec<i32> = vec![];
    a.push(1);
    println!("{:?}",a);
}
