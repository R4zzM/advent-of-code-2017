mod advent1;
mod advent2;

fn main() {
    let captcha1 = advent1::first_answer();
    let captcha2 = advent1::second_answer();
	println!("1st of december:");
    println!("First Captcha: {}", captcha1);
    println!("Second Captcha: {}", captcha2);
}