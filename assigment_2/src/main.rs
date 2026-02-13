
fn is_even(n: i32)->bool{
//if the number is divisible by two is even(true) else is odd(false)
    if n % 2 ==0 {
        return true;
    }else{
        return false;
    }
}
fn main(){
    let nums = [ 3,6,17,15,123,45,7,58,315,96 ]; //10 integers of my choice
    let mut sum = 0; //the sum starts at 0(initialize)
    let mut counter  = 0; //counter starts at 0(initialize)
    let mut i = 0;
    let mut greatest_num = nums[0];


    for n in nums{
      let div_3 = n % 3 == 0;
      let div_5 = n % 5 == 0;
      
      if is_even(n){
        println!("Its Even: {}", n );
      }else{
        println!("Its Odd: {}", n);
      }
        
      if div_3 && div_5{
        println!("FizzBuzz : {}" , n);
      }else if div_3{
        println!("Fizz : {}", n);
      }else if div_5 {
        println!("Buzz : {}", n);
      }else{
        println!("Not divisible : {}" , n)
      }
      println!("#########################"); //easier to separate 
    }
  while counter < nums.len(){
    sum += nums[counter];
    counter += 1;
  }
  println!("Total sum:  {}", sum);
  loop {
    //compares current number with the greatest
    if nums[i] > greatest_num {
        greatest_num = nums[i]; //updates the greatest number
    }
    i += 1; //moves to the next index
   
    if i == nums.len(){
        break; //stops loop when the index reaches the length of the array(10==10 )
    }

  }
  println!("The greatest number is: {}", greatest_num);
}