use rand::{self, Rng};
use std::{thread::{self}, str::Split};
use text_io::{self, read};
fn count() -> i32 {
    let mut rng = rand::thread_rng();
    //for i in 0..10000000 {
    let mut prvi:i32=0;
    let mut count:i32=0;
    for _  in 0i64..10000000000 {
        let a: u32= (rng.gen::<u32>())%10;
        //println!("{}",a);
        if a as i32 == prvi{
            prvi+=1;
            if prvi == 3 {
                //println!("{}",prvi+1);
                prvi=-1;
                count+=1;
                //println!("HIT");
            }
        }else{
            prvi=0;
        }
        
    }
    return count;
}

fn count2() -> i32 {
    let mut rng = rand::thread_rng();
    //for i in 0..10000000 {
    let mut rezultati = vec![0,1,2];
    let mut counter = 0;
    for _  in 0i64..10000000000 {
        let a: u32= (rng.gen::<u32>())%10;
        //println!("{}",a);
        if rezultati.contains(&a){
            //print!("{a}");
            //let pozicija = rezultati.iter().position(|&r| r == a).unwrap();
            //rezultati.remove(pozicija);
            rezultati.retain(|&x| x != a);
            if rezultati.len() == 0 {
                counter += 1;
                rezultati = vec![0,1,2];
            }
        }else{
            rezultati = vec![0,1,2];
        }
        
    }
    return counter;
}

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

    //random
    /*let mut res: [i32; 10] = [0; 10];
    let mut yolo = vec![]; 
    for _ in 0..10 {
        yolo.push(thread::spawn(move || count2() ));
    }
    let mut counte = 0;
    for y in yolo {
        res[counte] = y.join().unwrap();
        counte+=1;
    }
    let sum: i32 = res.iter().sum();
    println!("{sum}");
    println!("{}",  sum as f64/10000000000.0/10.0 )*/
}
