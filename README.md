# Shuffle
Small program to shuffle text file.

## Quick Start
```
git clone https://github.com/RoeyFuchs/shuffle
cd shuffle
cargo build --release
./target/release/shuffle < example.txt
```

## Options
use ```-h``` for help messege.
```
USAGE:
    shuffle [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input file>       input file. as default uses stdin
    -o, --output <output file>     output file. as default uses stdout
    -s, --separator <separator>    separator char or string. as deault uses new line
```

for example: ``` shuffle -i input.txt -o output.txt -s "," ```

## Use shuffle from everywhere
You need to add Shuffle's path as a search path. I prefer to add the executable program to /usr/local/bin:

``` sudo cp ./target/release/shuffle /usr/local/bin/ ```

## License
This code uses crates that under MIT license and Apache (2.0) license. 
