const NO_MORE_BOTTLES_OF_BEER: &str = concat!(
    "No more bottles of beer on the wall, no more bottles of beer.\n",
    "Go to the store and buy some more, 99 bottles of beer on the wall.\n"
);

const ONE_BOTTLE_OF_BEER: &str = concat!(
    "1 bottle of beer on the wall, 1 bottle of beer.\n",
    "Take it down and pass it around, no more bottles of beer on the wall.\n"
);

fn n_bottles_of_beer(n: u32) -> String {
    format!(
        "{} bottles of beer on the wall, {} bottles of beer.\n\
        Take one down and pass it around, {} bottle{} of beer on the wall.\n",
        n,
        n,
        n - 1,
        if n == 2 { "" } else { "s" },
    )
}

pub fn verse(n: u32) -> String {
    match n {
        2.. => n_bottles_of_beer(n),
        1 => ONE_BOTTLE_OF_BEER.to_owned(),
        0 => NO_MORE_BOTTLES_OF_BEER.to_owned(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
