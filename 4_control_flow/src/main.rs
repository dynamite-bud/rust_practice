use std::time::{Duration, Instant};

// the break keyword also returns a value and can be used to get the value from a loop assignment

fn main() {
    // let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // for num in &mut numbers {
    //     *num *= *num;
    // }

    // for num in numbers.iter_mut() {
    //     *num *= *num;
    // }

    // for i in 0..numbers.len() {
    //     println!("number {} is {}", i, numbers[i]);
    // }

    // println!("{:?}", numbers);

    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

    /*
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 100 {
                    break 'outer;
                }
            }
        }
    }
    */

    let n = 123456;
    let description = match n & 1 == 0 {
        true => "even",
        false => "odd",
    };
    println!("{} is {}", n, description);
}
