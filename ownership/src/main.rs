fn main() {
    let x = 5;
    let y = x;
    println!("X: {:?}, Y: {:?}",x,y);

    let s1 = "Dursun";
    let s2 = s1;
    println!("s1: {:?}, s2: {:?}",s1,s2);

    let ss1 = String::from("Dursun");
    let ss2 = ss1.clone(); //copying would cause a borrowing error!
    println!("ss1: {:?}, ss2: {:?}",ss1,ss2);

    let ll = calculate_len(&ss2);
    println!("ss2: {:?} --> len: {:?}",ss2,ll);

    let mut stamming = String::from("ZEGROS");
    
    stamp_it(&mut stamming);

    println!("{:?}", stamming);
}


fn calculate_len(s: &String) -> usize{
    s.len()
}

fn stamp_it(s :&mut String){
    s.push_str("-{stampped!}")
}