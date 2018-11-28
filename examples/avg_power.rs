extern crate clap;
extern crate failure;
extern crate garminfit as fit;

use std::{
    fs::File,
    io::BufReader,
};

fn main() {
    // Create the tiny CLI
    let matches = clap::App::new("garminfit: average power example")
        .about("calculate average power for a provided fit file")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("the .fit file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())
        .expect("provided file to exist");

    let mut reader = BufReader::new(file);

    match fit::File::decode(&mut reader) {
        Ok(decoded) => {
            for record in decoded.records {
                match record.content {
                    fit::record::Message::Data(fit::record::Data(mesgs)) => {
                        for mesg in mesgs {
                            match mesg {
                                fit::messages::Message::Record(
                                    fit::messages::Record::Power(pwr),
                                ) => println!("{:?}", pwr),
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
            }
        },
        Err(err) => eprintln!("{}", pretty_error(&err.into())),
    };
}

/// Return a prettily formatted error, including its entire
/// causal chain.
fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }
    pretty
}
