#[macro_use]
extern crate clap;
extern crate failure;
extern crate garminfit_profile_gen as profile_gen;

use std::{
    fs::File,
    io::{
        BufReader,
        Write,
    },
    path::PathBuf,
    process,
};

use profile_gen::{
    error::Error,
    open_workbook,
    worksheet,
    TokenStream,
    Xlsx,
};

fn main() {
    // CLI is specified in a yaml file. We load the matches and
    // then use the `From<clap::ArgMatches>` impl to get a neat
    // struct of parsed options (see `Options` type below).
    let yaml = load_yaml!("cli.yaml");
    let opt = clap::App::from_yaml(yaml).get_matches().into();
    if let Err(err) = try_main(opt) {
        // Print the error, including all of its underlying causes
        eprintln!("{}", pretty_error(&err));

        // If we get a non-empty backtrace (e.g., RUST_BACKTRACE=1
        // is set), then show it.
        let backtrace = err.backtrace().to_string();
        if !backtrace.trim().is_empty() {
            eprintln!("{}", backtrace);
        }
        process::exit(1);
    }
    // process::exit(0);
}

fn try_main(opt: Options) -> Result<(), failure::Error> {
    // Open the Profile.xlsx file passed in by the caller
    let mut workbook = open_workbook(opt.profile_xlsx)?;

    // Try and generate the module token streams
    let (types_tokens, mesgs_tokens) =
        generate_modules(&mut workbook, &opt.fit_sdk_version)?;

    // Create and write the types module
    let types_module_path = opt.output_dir.join(opt.types_module_path);
    File::create(&types_module_path)?
        .write_all(&types_tokens.to_string().into_bytes())?;
    println!("✓ {}", types_module_path.to_string_lossy());

    // Create and write the messages module
    let mesgs_module_path = opt.output_dir.join(opt.mesgs_module_path);
    File::create(&mesgs_module_path)?
        .write_all(&mesgs_tokens.to_string().into_bytes())?;
    println!("✓ {}", mesgs_module_path.to_string_lossy());

    // All done, nothing to return
    Ok(())
}

/// This function stitches together all the functionality
/// of this crate to provide two token streams: one for the
/// types module and one for the
/// messages module (respectively)
fn generate_modules(
    workbook: &mut Xlsx<BufReader<File>>,
    fit_sdk_version: &str,
) -> Result<(TokenStream, TokenStream), failure::Error> {
    // Process the "Types" worksheet
    let types_sheet = worksheet::types::open_sheet(workbook)?;
    let types = worksheet::types::extract(&types_sheet);
    let types_tokens =
        worksheet::types::generate_module(fit_sdk_version, &types)?;

    // Drill into the message numbers.
    let mesg_num_type = types.iter().find(|ty| ty.name == "MesgNum");

    let mesg_nums = match mesg_num_type {
        Some(worksheet::types::Type {
            values, ..
        }) => {
            values.iter().map(|v| (v.name.clone(), v.value.clone())).collect()
        },
        None => return Err(Error::missing_type("MesgNum".to_string()).into()),
    };

    // Process the "Messages" worksheet
    let mesgs_sheet = worksheet::messages::open_sheet(workbook)?;
    let mesgs = worksheet::messages::extract(&mesgs_sheet);
    let mesgs_tokens = worksheet::messages::generate_module(
        fit_sdk_version,
        &mesgs,
        mesg_nums,
    )?;

    // Ok, return token streams
    Ok((types_tokens, mesgs_tokens))
}

/// Command line options, all in a single `struct`.
struct Options {
    profile_xlsx:      PathBuf,
    output_dir:        PathBuf,
    types_module_path: PathBuf,
    mesgs_module_path: PathBuf,
    fit_sdk_version:   String,
}

// TODO: should this be TryFrom?
impl<'a> From<clap::ArgMatches<'a>> for Options {
    fn from(matches: clap::ArgMatches<'a>) -> Self {
        Options {
            profile_xlsx:      PathBuf::from(
                matches.value_of("profile_xlsx").expect("required argument"),
            ),
            output_dir:        PathBuf::from(
                matches.value_of("output_dir").expect("required argument"),
            ),
            types_module_path: PathBuf::from(
                matches.value_of("types_module_path").unwrap_or("types.rs"),
            ),
            mesgs_module_path: PathBuf::from(
                matches.value_of("mess_module_path").unwrap_or("messages.rs"),
            ),
            fit_sdk_version:   matches
                .value_of("fit_sdk_version")
                .unwrap_or("UNKNOWN")
                .to_string(),
        }
    }
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
