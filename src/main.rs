use owo_colors::AnsiColors::{Black, Green, Red, Yellow};
use owo_colors::OwoColorize;

fn main() {
    println!(
        "{} {} {}",
        "Red".color(Red),
        "Green".color(Green),
        "Yellow".color(Yellow)
    );
    println!("{}", "Red BG".on_color(Red));
    println!("{}", "Green BG".on_color(Green));
    println!("{}", "Yellow BG".on_color(Yellow));
    println!("{}", "Red BG, Black FG".on_color(Red).color(Black));
    println!("{}", "Green BG, Black FG".on_color(Green).color(Black));
    println!("{}", "Yellow BG, Black FG".on_color(Yellow).color(Black));
    println!("");
    println!(
        "{}{}{}",
        "Hello".color(Red),
        " ".on_color(Red),
        "World".color(Red)
    );
    println!(
        "{}{}{}",
        "Hello".color(Green),
        " ".on_color(Green),
        "World".color(Green)
    );
    println!(
        "{}{}{}",
        "Hello".color(Yellow),
        " ".on_color(Yellow),
        "World".color(Yellow)
    );
}
