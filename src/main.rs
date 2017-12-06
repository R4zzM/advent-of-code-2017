mod adventhelper;
// mod advent1;
// mod advent2;
// mod advent3;
// mod advent4;
// mod advent5;
mod advent6;

fn main() {
 //    let captcha1 = advent1::first_answer();
 //    let captcha2 = advent1::second_answer();
	// println!("1st of december:");
 //    println!("First Captcha: {}", captcha1);
 //    println!("Second Captcha: {}", captcha2);

 //    let cksum1 = advent2::first_answer();
 //    let cksum2 = advent2::second_answer();
 //    println!("2nd of december:");
 //    println!("First spreadsheet cksum: {}", cksum1);
 //    println!("Second spreadsheet cksum: {}", cksum2);

 //    let distance1 = advent3::first_answer();
 //    let distance2 = advent3::second_answer();
 //    println!("3nd of december:");
 //    println!("First distance: {}", distance1);
 //    println!("Second distance: {}", distance2);

 //    let valid_passwds = advent4::first_answer();
 //    let valid_passwds2 = advent4::second_answer();
 //    println!("4th of december:");
 //    println!("Valid passwords: {}", valid_passwds);
 //    println!("Valid passwords2: {}", valid_passwds2);

    // let jumps = advent5::first_answer();
    // let jumps2 = advent5::second_answer();
    // println!("5th of december:");
    // println!("Jumps1: {}", jumps);
    // println!("Jumps2: {}", jumps2);

    {
        let (first, second) = advent6::both();
        println!("6th of december:");
        println!("First: {}", first);
        println!("Second: {}", second);
    }
    
}