use ansi_term::Colour::{Black, Blue, Cyan, Green, Red, Yellow};
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
        "How about some {} and {}?",
        Style::new().bold().paint("bold"),
        Style::new().underline().paint("underline")
    );

    println!(
        "Yellow on blue: {}",
        Style::new().on(Blue).fg(Yellow).paint("yow!")
    );
    println!(
        "Also yellow on blue: {}",
        Cyan.on(Blue).fg(Yellow).paint("zow!")
    );

    println!(
        "{} {} {}",
        Red.paint("Red"),
        Green.paint("Green"),
        Yellow.paint("Yellow")
    );
    println!("{}", Red.on(Red).fg(Black).paint("Red BG"));
    println!("{}", Green.on(Green).fg(Black).blink().paint("Green BG"));
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
        Style::new().on(Yellow).fg(Black).blink().paint(" "),
        Yellow.paint("World")
    );
    println!("---------------------");
    println!(
        "{}{}{}",
        Red.paint("Hello"),
        Style::new().on(Red).fg(Black).underline().paint(" "),
        Red.paint("underline")
    );
    println!(
        "{}{}{}",
        Blue.paint("Hello"),
        Style::new().on(Blue).fg(Black).dimmed().paint(" "),
        Blue.paint("dimmed")
    );
    println!(
        "{}{}{}",
        Green.paint("Hello"),
        Style::new().on(Green).fg(Black).blink().paint(" "),
        Green.paint("blink")
    );
    println!(
        "{}{}{}",
        Cyan.paint("Hello"),
        Style::new().on(Cyan).fg(Black).italic().paint(" "),
        Cyan.paint("italic")
    );
    println!(
        "{}{}{}",
        Yellow.paint("Test"),
        Style::new().on(Yellow).fg(Black).paint("ㅤ"),
        Yellow.paint("U+3164")
    );
}
