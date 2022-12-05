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

fn _day4(){
    let mut count = 0;
    while true {
        let mut unos: String = read!("{}\n");
        if unos == "KRAJ" { break; }
        let mut prviod = unos.split(",").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let mut prvido = unos.split(",").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let mut drugiod = unos.split(",").collect::<Vec<&str>>()[1].split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let mut drugido = unos.split(",").collect::<Vec<&str>>()[1].split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        //create array with numbers ranging from x to y
        let mut array = (prviod..prvido+1).collect::<Vec<i32>>();
        let mut array2 = (drugiod..drugido+1).collect::<Vec<i32>>();
        //println!("\n{:?} {:?}\n", array, array2);

        for x in array {
            if array2.contains(&x) {
                count += 1;
                break
            }
        }
        /* 
        if (prviod <= drugiod || prvido >= drugido) 
        || (drugiod<= prviod  || drugido>=prvido) {
            count +=1;
            println!("\nGOTCHA {}-{}  {}-{}\n",prviod,prvido,drugiod,drugido);
        }*/
    }
    println!("{}",count);
}

fn day5(){
    let mut unos1: String = read!("{}\n");
    let mut unos2: String = read!("{}\n");
    let mut unos3: String = read!("{}\n");
    let mut unos4: String = read!("{}\n");
    let mut unos5: String = read!("{}\n");
    let mut unos6: String = read!("{}\n");
    let mut unos7: String = read!("{}\n");
    let mut unos8: String = read!("{}\n");
    let mut unos9: String = read!("{}\n");
    let mut skupina = [unos1,unos2,unos3,unos4,unos5,unos6,unos7,unos8,unos9];
    let mut unos: String = read!("{}\n");
    while unos != "KRAJ" {
        let mut inputProcessed = unos.split(" ").collect::<Vec<&str>>();
        let koliko = inputProcessed[0].parse::<i32>().unwrap();
        let odkud = inputProcessed[1].parse::<i32>().unwrap();
        let dokud = inputProcessed[2].parse::<i32>().unwrap();
        let mut tmp: String = String::new();
        for _ in 0..koliko {
            tmp.push(skupina[(odkud-1) as usize].pop().unwrap());
            
        }
        for _ in 0..koliko {
            skupina[(dokud-1) as usize].push(tmp.pop().unwrap());
        }
        unos = read!("{}\n");
    }
    for x in skupina {
        print!("{}",x.get((x.len()-1) as usize..).unwrap());
    }
    println!("");
}

fn main() {
    day5()
}
