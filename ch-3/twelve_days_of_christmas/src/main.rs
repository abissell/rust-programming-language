fn main() {
    for day in 1 .. 13 {
        print_day(day);
    }
}

fn print_day(mut day: u32) {
    let mut lyric: String = "On the ".to_string();
    lyric.push_str(num_to_english(day));
    lyric.push_str(" day of Christmas, my true love gave to me:\n");
    while day > 0 {
        lyric.push_str(num_to_lyric(day));
        if day == 2 {
            lyric.push_str("and ");
        }
        day -= 1;
    }

    println!("{}", lyric);
}

fn num_to_english(num: u32) -> &'static str {
    match num {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => panic!("Cannot translate {} to ordinal English!", num),
    }
}

fn num_to_lyric(num: u32) -> &'static str {
    match num {
        1 => "a partridge in a pear tree.\n",
        2 => "two turtle doves,\n",
        3 => "three French hens,\n",
        4 => "four calling birds,\n",
        5 => "five gold rings,\n",
        6 => "six geese-a-laying,\n",
        7 => "seven swans-a-swimming,\n",
        8 => "eight maids-a-milking,\n",
        9 => "nine ladies dancing,\n",
        10 => "ten lords-a-leaping,\n",
        11 => "eleven pipers piping,\n",
        12 => "twelve drummers drumming,\n",
        _ => panic!("Cannot translate {} to lyric!", num),
    }
}
