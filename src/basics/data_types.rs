pub fn read() {
    println!("Hello, world read2!");
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut altitude = 0;

    for i in gain {
        altitude += i;
        if altitude > max {
            max = altitude;
        }
    }
    return max;
}


pub fn can_flower(mut flower_bed: Vec<i32>, n: i32,) -> bool {
    // (a < b && b < c) || (a > b && b > c)
    let mut empty_plots  = 0;
    let length = flower_bed.len();

    for i in 0..length {
        let current_plot = flower_bed[i];
        if(current_plot == 0) {
           let empty_prev_plot  =  (i == 0) || (flower_bed[i - 1] == 0);
           let empty_next_plot = (i == length - 1 )|| (flower_bed[i + 1] == 0);
           if(empty_prev_plot && empty_next_plot) {
               flower_bed[i] = 1;
               empty_plots += 1;
           }
        }
    }


    return empty_plots >= n;
}