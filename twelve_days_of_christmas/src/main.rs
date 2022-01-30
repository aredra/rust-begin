// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree

// On the second day of Christmas, my true love sent to me
// Two turtledoves
// And a partridge in a pear tree

// On the third day of Christmas, my true love sent to me
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the fourth day of Christmas, my true love sent to me
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the fifth day of Christmas, my true love sent to me
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the sixth day of Christmas, my true love sent to me
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the seventh day of Christmas, my true love sent to me
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the eighth day of Christmas, my true love sent to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the ninth day of Christmas, my true love sent to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the tenth day of Christmas, my true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the eleventh day of Christmas, my true love sent to me
// I sent eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// On the twelfth day of Christmas, my true love sent to me
// Twelve drummers drumming
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree

// And a partridge in a pear tree
const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
const PRESENTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtledoves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings (five golden rings)",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    for day in 0..DAYS.len() {
        // let mut index = 0; // 이거 안됨.. 소유권 때문인가..
        println!("On the {} day of Christmas, my true love sent to me", DAYS[day]);

        for n in (0..day+1).rev() {
            if day != 0 && n == 0 {
                println!("And a partridge in a pear tree");
            } 
            else {
                println!("{}", PRESENTS[n]);
            }
  
        }
        println!("");
    }
    println!("And a partridge in a pear tree");
}
