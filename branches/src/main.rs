fn main() {
    for number in 1..4 {
        println!("{number}");
    }
    // 反转
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

fn loop_tag() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // 由于在内层loop逻辑，指定外层loop（利用标签），break指定loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}