use text_io::{self, read};


fn day2(){
    let mut calc = 0;
    while true {
        let mut input: String = read!("{}\n");
        //println!("{}",input.get(2..).unwrap());
        if input.get(0..1)==Some("A"){
            match input.get(2..) {
                Some(x) if x == "X" => calc += 3,
                Some(x) if x == "Y" => calc += 1+3,
                Some(x) if x == "Z" => calc += 2+6,
                Some(_) => todo!(),
                None => todo!(),
            }
        }
        if input.get(0..1)==Some("B"){
            match input.get(2..) {
                Some(x) if x == "X" => calc += 1,
                Some(x) if x == "Y" => calc += 2+3,
                Some(x) if x == "Z" => calc += 3+6,
                Some(_) => todo!(),
                None => todo!(),
            }
        }
        if input.get(0..1)==Some("C"){
            match input.get(2..) {
                Some(x) if x == "X" => calc += 2,
                Some(x) if x == "Y" => calc += 3+3,
                Some(x) if x == "Z" => calc += 1+6,
                Some(_) => todo!(),
                None => todo!(),
            }
        }
        if input.eq("END"){
            break;
        }   
    }
    println!("{}",calc);
}


fn main() {
    day2();
}
