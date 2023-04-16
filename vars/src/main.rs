fn main() {
    let tup = ("Dursun", 41);
    let (name, age) = tup;
    println!("{:?}", tup);
    println!("name: {}, age:{}", name, age);
    let isim = tup.0;
    let yas = tup.1;
    println!("İsim: {}, yaş:{}", isim, yas);

    let numbers = vec![1, 2, 3, 4, 5, 6];
    for n in numbers {
        println!("{:?}",n);
    }

    for n in 1..5{
        println!("{:?}",n);
    }
}
