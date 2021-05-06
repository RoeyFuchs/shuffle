pub mod config {
    use super::opt::Opt;
    use std::fs::File;
    use std::io::{self, Read, Write};

    pub struct Config {
        pub source: Box<dyn Read>,
        pub separator: String,
        pub destinition: Box<dyn Write>,
    }

    impl Config {
        pub fn new(args: &Opt) -> Result<Self, std::io::Error> {
            let src: Box<dyn Read> = match &args.input {
                Some(x) => Box::new(File::open(x)?),
                None => Box::new(io::stdin()),
            };
            let des: Box<dyn Write> = match &args.output {
                Some(x) => Box::new(File::create(x)?),
                None => Box::new(io::stdout()),
            };

            let sep: String = String::from(args.sep.to_str().unwrap()); // it is safe to unwrap. this Option will never be None, since this arttivute has a default value.
            Ok(Config {
                source: src,
                separator: sep,
                destinition: des,
            })
        }
    }
}

pub mod string_manipulation {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    use std::io::Write;
    pub fn shuffle_vec<T>(vec: &mut Vec<T>) {
        vec.shuffle(&mut thread_rng());
    }
    pub fn separat_string(buffer: &String, separator: &String) -> Vec<String> {
        return buffer
            .split(separator)
            .map(str::to_string)
            .map(|s| String::from(s.trim()))
            .filter(|s| !s.is_empty())
            .collect();
    }
    pub fn write_to_stream(
        strings_vec: &Vec<String>,
        writer: &mut Box<dyn Write>,
    ) -> std::io::Result<()> {
        for s in strings_vec {
            writeln!(writer, "{}", &s)?;
        }
        Ok(())
    }
}
pub mod opt {
    use std::path::PathBuf;
    use structopt::StructOpt;
    #[derive(StructOpt, Debug)]
    #[structopt(about = "A basic program to shuffle text file" , author = "Roey Fuchs")]
    pub struct Opt {
        #[structopt(
            short,
            long,
            help = "input file. as default uses stdin",
            name = "input file",
            parse(from_os_str)
        )]
        pub input: Option<PathBuf>,

        #[structopt(
            short,
            long,
            help = "output file. as default uses stdout",
            name = "output file",
            parse(from_os_str)
        )]
        pub output: Option<PathBuf>,

        #[structopt(
            short,
            long,
            help = "separator char or string. as deault uses new line",
            name = "separator",
            long = "separator",
            hide_default_value = true,
            default_value = "\n",
            parse(from_os_str)
        )]
        pub sep: PathBuf,
    }
}
