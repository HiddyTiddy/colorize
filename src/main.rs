use std::fmt::Write;
use std::io::{self, BufRead};

use ansi_term::{Colour, Style};
use clap::{ArgEnum, Parser};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

impl Color {
    fn to_colour(self) -> Colour {
        match self {
            Color::Red => Colour::Red,
            Color::Green => Colour::Green,
            Color::Blue => Colour::Blue,
            Color::Black => Colour::Black,
            Color::Yellow => Colour::Yellow,
            Color::Purple => Colour::Purple, // why he ourple :skull:
            Color::Cyan => Colour::Cyan,
            Color::White => Colour::White,
        }
    }
}

#[derive(Parser)]
#[clap(
    about = "Color some text",
    long_about = "colors given text\nthis requires a terminal that supports ANSI colors"
)]
#[clap(author, version)]
struct App {
    #[clap(short, long)]
    #[clap(arg_enum)]
    /// the foreground color of the text
    color: Option<Color>,

    #[clap(short, long)]
    #[clap(arg_enum)]
    /// the background color of the text
    on: Option<Color>,

    #[clap(short = 'b', long)]
    /// whether the text is bold
    bold: bool,

    #[clap(short, long)]
    /// whether the text is dimmed
    dimmed: bool,

    #[clap(short = 'B', long)]
    /// whether the text is blinking
    blink: bool,

    #[clap(short, long)]
    /// whether the text is strikethrough
    strike: bool,

    #[clap(short, long)]
    /// whether the text is underlined
    underline: bool,

    #[clap(short, long)]
    /// whether the text style is reversed
    reverse: bool,

    #[clap(short = 'H', long)]
    /// whether the text style is hidden
    hidden: bool,

    /// the text to which the style is applied
    text: Vec<String>,
}

fn main() -> color_eyre::eyre::Result<()> {
    let args = App::parse();

    let text = match args.text.is_empty() {
        false => args.text.join(" "),
        true => {
            let stdin = io::stdin();
            let mut text = String::new();
            for line in stdin.lock().lines() {
                let line = line?;
                writeln!(&mut text, "{}", line)?;
            }

            text
        }
    };

    let mut style = Style::new();
    match args.color {
        None => (),
        Some(color) => {
            style = style.fg(color.to_colour());
        }
    }

    match args.on {
        None => (),
        Some(color) => {
            style = style.on(color.to_colour());
        }
    }

    if args.bold {
        style = style.bold();
    }

    if args.dimmed {
        style = style.dimmed();
    }

    if args.blink {
        style = style.blink();
    }

    if args.strike {
        style = style.strikethrough();
    }

    if args.underline {
        style = style.underline();
    }

    if args.reverse {
        style = style.reverse();
    }

    if args.hidden {
        style = style.hidden();
    }

    print!("{}", style.paint(text));

    Ok(())
}
