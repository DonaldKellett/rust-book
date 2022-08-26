const DAYS: [&str; 11] = [
    "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const PRESENTS: [&str; 11] = [
    "Two turtle doves",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    println!("On the first day of Christmas my true love sent to me");
    println!("A partridge in a pear tree");
    for i in 0..11 {
        println!();
        println!(
            "On the {} day of Christmas my true love sent to me",
            DAYS[i]
        );
        for j in (0..=i).rev() {
            println!("{}", PRESENTS[j]);
        }
        println!("And a partridge in a pear tree");
    }
}
