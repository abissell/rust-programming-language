fn main() {
    let args = std::env::args();
    let num_args = args.len();
    if num_args != 2 {
        println!("USAGE: Enter a temperature followed by 'F' or 'C' to \
                  convert to Celsius / Fahrenheit.");
        std::process::exit(1);
    }

    let mut temp = args.skip(1).next().expect("Should check for second arg!");
    let scale = temp.pop().unwrap();
    let orig_temp = temp.parse::<f64>().unwrap();
    let (new_temp, new_scale) = match scale {
        'f' | 'F' => ((orig_temp - 32.) * (5./9.), 'C'),
        'c' | 'C' => (orig_temp * (9./5.) + 32., 'F'),
        _ => panic!("Could not properly parse scale {}", scale),
    };

    println!("{}{} = {}{}", orig_temp, scale, new_temp, new_scale);
}
