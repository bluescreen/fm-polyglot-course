enum RSEnum {
    Foo(i32),
    Bar(String),
    Baz(Vec<String>)
}


enum Foo<T> {
    Bar(T)
}

fn main() {
    let mut a:Vec<i32> = vec![];
    a.push(1);
    println!("{:?}",a);


    let foo = RSEnum::Foo(5);

    if let RSEnum::Foo(value) = foo {

    }

    match foo{
        RSEnum::Foo(value) => {},
        _ => {}
    }
}
