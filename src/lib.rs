#![feature(try_from)]
#![feature(test)]
#![allow(dead_code)]

extern crate byteorder;
extern crate chrono;
extern crate failure;
extern crate test;

pub(crate) mod bits;
pub(crate) mod dyncrc16;

pub mod error;
pub mod profile;
pub mod types;

pub use types::{
    file::{
        self,
        File,
    },
    record,
};

pub use profile::messages;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{
            Cursor,
            Read,
        },
        path::Path,
    };
    use test::Bencher;

    // Benchmark FIT file decoding. Takes the std `Bencher` and
    // the path to a `.fit` file to decode. Reads the file once
    // then puts it behind a cursor for benchmarking.
    fn bench_file_decoding(b: &mut Bencher, file: &Path) {
        let mut file = File::open(file).unwrap();
        let data = {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            Cursor::new(bytes)
        };
        // NOTE: is it an issue that this benchmarks the clone?
        b.iter(|| super::File::decode(&mut data.clone()).unwrap());
    }

    macro_rules! bench_file {
        ($name:ident, $path:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                bench_file_decoding(
                    b,
                    &Path::new(env!("CARGO_MANIFEST_DIR")).join($path),
                );
            }
        };
    }

    bench_file!(bench_python_fitparse_1, "testdata/python_fitparse_1.fit");
    bench_file!(bench_python_fitparse_2, "testdata/python_fitparse_2.fit");

    // TODO: should get a NotFIT error when opening a non-fit
    // file.
}
