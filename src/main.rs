mod basics;

use crate::basics::data_types;

fn main() {
    println!("Hello, world!");
    data_types::read();
    let sum = data_types::sum(1, 2);
    println!("sum of the  = {}", sum);
    
    let gain = vec![-5,1,5,0,-7];
    let gain2: Vec<i32> = vec![-4,-3,-2,-1,4,3,2];
    println!("largest altitude = {}", data_types::largest_altitude(gain));
    println!("largest altitude = {}", data_types::largest_altitude(gain2));


    //canflower
    let mut flowerbed  = vec![1,0,0,0,1];
    let n = 1;
    print!("can flower = {} \n", data_types::can_flower(flowerbed, n));


    let mut flowerbed2  = vec![1,0,0,0,1];
    let n2 = 2;
    print!("can flower = {}", data_types::can_flower(flowerbed2, n2));
}