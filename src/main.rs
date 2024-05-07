// mod leetcode75;

// use crate::leetcode75::leet_75;

mod leetcode;
use crate::leetcode::leet_75;

fn main() {
    println!("Hello, world!");
    leet_75::read();
    let sum = leet_75::sum(1, 2);
    println!("sum of the  = {}", sum);
    
    let gain = vec![-5,1,5,0,-7];
    let gain2: Vec<i32> = vec![-4,-3,-2,-1,4,3,2];
    println!("largest altitude = {}", leet_75::largest_altitude(gain));
    println!("largest altitude = {}", leet_75::largest_altitude(gain2));


    //canflower
    let mut flowerbed  = vec![1,0,0,0,1];
    let n = 1;
    print!("can flower = {} \n", leet_75::can_flower(flowerbed, n));


    let mut flowerbed2  = vec![1,0,0,0,1];
    let n2 = 2;
    print!("can flower = {}", leet_75::can_flower(flowerbed2, n2));
}