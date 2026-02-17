fn check_guess(guess: i32, secret: i32)-> i32{
    if guess == secret{
        return 0;
    }else if guess > secret {
        return 1;
    }else{
        return -1;
    }
    
}

fn main() {
    let mut secret_num = 17;
    let mut counter = 0;//initialize
    //add a loop
    let mut guess_num = 13;
    loop {
        
        let result = check_guess(guess_num, secret_num);
        
        if result == 0 {
            println!("Your guess is correct.");
            break;//break until the guess = secret are equal 
        }else if result == 1{
            guess_num -= 1;//will subtract each iteration of our guess num if its greater then our secret num
            println!("Your guess is too high.");
            
        }else{
            guess_num += 1; //will add each iteration of our guess num when its a smaller number then our secret num
            println!("Your guess is too low.");
            
        }
        counter += 1;//sum
    
  } 
  println!("Total of guesses: {}",counter);
}
