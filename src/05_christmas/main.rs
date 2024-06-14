const VERSES: usize = 12;

const DAYS: [&str; VERSES] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const GIFTS: [&str; VERSES] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

const FIRST_GIFT: &str = "A partridge in a pear tree.\n";

macro_rules! song_format {
    () => {
        "On the {0} day of Christmas,\n\
         my true love gave to me\n\
         {1}"
    };
}

fn main() {

    println!("Let me sing!\n");
    println!(song_format!(), DAYS[0], FIRST_GIFT);
    for day in 1..VERSES-1 {
        println!(song_format!(), DAYS[day], get_gifts(day) + ".\n");
    }
    println!(song_format!(), DAYS[11], get_gifts(11) + "!\n");
}

fn get_gifts(day: usize) -> String {

    let mut gifts = String::new();

    for day in (1..=day).rev() {
        gifts.push_str(GIFTS[day]);
        gifts.push_str(",\n");
    }

    gifts.push_str(GIFTS[0]);
    gifts
}