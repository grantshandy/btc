use clap::{App, Arg, crate_version, crate_name, crate_authors, crate_description};
use coindesk::Bitcoin;
use colored::Colorize;

#[async_std::main]
async fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("usd")
            .short("u")
            .long("usd")
            .help("Print the price in United States Dollars (default)")
            .conflicts_with("gbp")
            .conflicts_with("eur")
        )
        .arg(Arg::with_name("gbp")
            .short("g")
            .long("gbp")
            .help("Print the price in British Pound Sterlings")
            .conflicts_with("usd")
            .conflicts_with("eur")
        )
        .arg(Arg::with_name("eur")
            .short("e")
            .long("eur")
            .help("Print the price in Euros")
            .conflicts_with("usd")
            .conflicts_with("gbp")
        )
        .arg(Arg::with_name("simple")
            .short("s")
            .long("simple")
            .help("Print the price without the time or currency symbol")
        )
        .get_matches();

    let data = match Bitcoin::get().await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("{} {}\n", "error:".red().bold(), error);
            eprintln!("USAGE:\n    btc [FLAGS]\n");
            eprintln!("For more information try {}", "--help".green());
            std::process::exit(1);
        }
    };

    if app.is_present("simple") {
        if app.is_present("usd") {
            println!("{}", data.usd.rate);
        } else if app.is_present("gbp") {
            println!("{}", data.gbp.rate);
        } else if app.is_present("eur") {
            println!("{}", data.eur.rate);
        } else {
            println!("{}", data.usd.rate);
        };
    } else {
        println!("{}", data.time.to_string().green());

        if app.is_present("usd") {
            println!("${}", data.usd.rate);
        } else if app.is_present("gbp") {
            println!("£{}", data.gbp.rate);
        } else if app.is_present("eur") {
            println!("€{}", data.eur.rate);
        } else {
            println!("${}", data.usd.rate);
        };
    };
}
