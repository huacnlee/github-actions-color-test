use ansi_term::Colour::{Black, Green, Red, Yellow};
use ansi_term::Style;

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
    // println!(
    //     "{}{}{}",
    //     "Hello".color(Red),
    //     "  ".on_red(),
    //     "World".color(Red)
    // );
    // println!(
    //     "{}{}{}",
    //     "Hello".color(Red),
    //     " ".on_color(Red),
    //     "World".color(Red)
    // );
    // println!(
    //     "{}{}{}",
    //     "Hello".color(Green),
    //     " ".on_color(Green),
    //     "World".color(Green)
    // );
    // println!(
    //     "{}{}{}",
    //     "Hello".color(Yellow),
    //     " ".on_color(Yellow),
    //     "World".color(Yellow)
    // );
}
