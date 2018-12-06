# Advent of Code
Advent of Code in Rust, with some light explainations of interesting things. 

## Getting Started

### Prerequisites

This project is written in Rust, so if you'd like to run these snippets it
would be worth while to have that installed.

On Unix:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Of course, you shouldn't trust installers from the internet without reading
what it is they do first, and should nver pipe curl directly into bash, etc.

### Installing and Running

Installing is as simple as cloning this repo...

```sh
git clone https://github.com/dr-jonesy/aoc
```

...moving to the day you want...

```sh
cd aoc/day1
```

...and running the code. Each 'day' directory should have all the 
requisite files in the top level (aoc/day1/input.txt for example).


```sh
cargo run
```

### Day 1

#### Part One: Frequency Finder

This is on the face of it a simple little problem, almost more of a code kata
than anything else. It breaks down into three sections: taking input,
parsing that input for further use, and then doing something with that parsed
data.

One thing I learned here is how to do string indexing in Rust.

#### Part Two: Repeat as Needed


