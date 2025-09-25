use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value=26;
    let simulated_random_number=3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity:u32)->u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity:u32,random_number:u32){
    if intensity<25{
        println!(
            "Today, do {} pushups",
            simulated_expensive_calculation(intensity) 
        );
        print!(
            "Next, do {} situps ",
            simulated_expensive_calculation(intensity)
        );
    } else{
        if random_number ==3{
            print!("Take a break today! Remember to stay hydrated ");
        } else{
        print!(
            "Today, run for {} minutes ",
            simulated_expensive_calculation(intensity)
        );
        }
    }
}