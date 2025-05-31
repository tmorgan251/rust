fn main() {
    // Print each day's verses
    for day in 1..=12 {
        print_day_lyrics(day);
        println!(); // Empty line between verses
    }
}

fn print_day_lyrics(day: usize) {
    let day_names = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Print the opening line
    println!("On the {} day of Christmas,", day_names[day - 1]);
    println!("my true love sent to me");

    // Print gifts in reverse order (newest first)
    for gift_day in (1..=day).rev() {
        print_gift_line(gift_day, day);
    }
}

fn print_gift_line(gift_day: usize, total_days: usize) {
    // Use placeholder text to avoid copyright issues
    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six gees a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    if gift_day <= gifts.len() {
        // Handle the "and" before the last line when it's not the first day
        if gift_day == 1 && total_days > 1 {
            println!("And {}", gifts[gift_day - 1].to_lowercase());
        } else {
            println!("{}", gifts[gift_day - 1]);
        }
    }
}
