use text_io::{self, read};


fn _day2(){
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

fn day3(){
    let mut alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut alphabet_lower = "abcdefghijklmnopqrstuvwxyz";
    let mut count = 0;
    while true {
        let mut unos1: String = read!("{}\n");
        if unos1 == "KRAJ" { break; }
        let mut unos2: String = read!("{}\n");
        let mut unos3: String = read!("{}\n");
        let mut tmpCount = 0;
        for p in 0..unos1.len() {
            let char = unos1.get(p..p+1).unwrap();
            if unos2.contains(char) {
                if unos3.contains(char) {
                    if alphabet.contains(char) && (alphabet.find(char).unwrap_or(0)+27) > tmpCount {
                        tmpCount = (alphabet.find(char).unwrap_or(0)+27);
                        break;
                    }
                    if alphabet_lower.contains(char) && (alphabet_lower.find(char).unwrap_or(0)+1) > tmpCount {
                        tmpCount = alphabet_lower.find(char).unwrap_or(0)+1;
                        break;
                    }
                }
                //println!("{} {} {}" ,char,(alphabet.find(char).unwrap_or(0)+27), (alphabet_lower.find(char).unwrap_or(0)+1));
                
            }
        }
        /*
        pt1
        let mut unos: String = read!("{}\n");
        if unos == "KRAJ" { break; }
        let mut prvi = unos.get(0..(unos.len()/2)).unwrap();
        let mut drugi  = unos.get(unos.len()/2..).unwrap();
        let mut tmpCount = 0;
        for p in 0..drugi.len() {
            let char = drugi.get(p..p+1).unwrap();
            if prvi.contains(char) {
                //println!("{} {} {}" ,char,(alphabet.find(char).unwrap_or(0)+27), (alphabet_lower.find(char).unwrap_or(0)+1));
                if alphabet.contains(char) && (alphabet.find(char).unwrap_or(0)+27) > tmpCount {
                    tmpCount = (alphabet.find(char).unwrap_or(0)+27);
                    break;
                }
                if alphabet_lower.contains(char) && (alphabet_lower.find(char).unwrap_or(0)+1) > tmpCount {
                    tmpCount = alphabet_lower.find(char).unwrap_or(0)+1;
                    break;
                }
            }
        } */
        //println!("");
        count += tmpCount;
    }
    println!("{}",count);
}

fn main() {
    day3()
}
