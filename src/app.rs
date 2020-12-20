use clap::{App, Arg};

pub fn build() -> App<'static, 'static> {
  App::new("local")
    .version("version")
    .about("about")
    .arg(Arg::with_name("FILE").multiple(true).default_value("."))
    .arg(
      Arg::with_name("all")
        .short("a")
        .overrides_with("almost-all")
        .long("all")
        .multiple(true)
        .help("Do not ignore entries starting with ."),
    )
    .arg(
      Arg::with_name("color")
        .long("color")
        .possible_value("always")
        .possible_value("auto")
        .possible_value("never")
        .default_value("auto")
        .multiple(true)
        .number_of_values(1)
        .help("when to use terminal colors"),
    )
    .arg(
      Arg::with_name("icon")
        .long("icon")
        .possible_value("always")
        .possible_value("auto")
        .possible_value("never")
        .default_value("auto")
        .multiple(true)
        .number_of_values(1)
        .help("When to print the icons"),
    )
}
