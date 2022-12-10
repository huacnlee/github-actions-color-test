use ansi_term::Colour::{Black, Green, Red, Yellow};
use ansi_term::Style;
use owo_colors::OwoColorize;

pub fn test() {
    println!("\n---------------- ansi_term ----------------");
    println!(
        "How about some {} and {}?",
        Style::new().bold().paint("bold"),
        Style::new().underline().paint("underline")
    );

    println!(
        "Demonstrating {} and {}!",
        Green.bold().paint("blue bold"),
        Yellow.underline().paint("yellow underline")
    );

    println!(
        "{} {} {}",
        Red.paint("Red"),
        Green.paint("Green"),
        Yellow.paint("Yellow")
    );
    println!("{}", Red.on(Red).fg(Black).paint("Red BG"));
    println!("{}", Green.on(Green).fg(Black).paint("Green BG"));
    println!("{}", Yellow.on(Yellow).fg(Black).paint("Yellow BG"));
    println!("");
    println!(
        "{}{}{}",
        Red.paint("Hello"),
        Style::new().underline().fg(Red).paint(" "),
        Red.paint("World")
    );

    println!(
        "{}{}{}",
        Red.paint("Hello"),
        Style::new().on(Red).paint(" "),
        Red.paint("World")
    );
    println!(
        "{}{}{}",
        Green.paint("Hello"),
        Style::new().on(Green).fg(Black).paint(" "),
        Green.paint("World")
    );
    println!(
        "{}{}{}",
        Yellow.paint("Hello"),
        Style::new().on(Yellow).fg(Black).paint(" "),
        Yellow.paint("World")
    );
}
