# btc
A CLI tool for getting the price of bitcoin in different currencies.
```
cargo install btc
```

## Usage

Basic:
```
$ btc
2021-05-08 00:39:00 UTC
$57526.9906
```

Dollars:
```
$ btc -u
2021-05-08 00:41:00 UTC
$57497.2017
```

Euros:
```
$ btc -e
2021-05-08 00:40:00 UTC
€47271.2669
```

Pounds:
```
$ btc -g
2021-05-08 00:41:00 UTC
£41104.5195
```

Silent:
```
$ btc -s
57602.6239
```

## Command Line Options
```
btc 0.1.0
Grant Handy <grantshandy@gmail.com>
A CLI tool for getting the price of bitcoin in different currencies.

USAGE:
    btc [FLAGS]

FLAGS:
    -e, --eur        Print the price in Euros
    -g, --gbp        Print the price in British Pound Sterlings
    -h, --help       Prints help information
    -s, --simple     Print the price without the time or currency symbol
    -u, --usd        Print the price in United States Dollars (default)
    -V, --version    Prints version information
```