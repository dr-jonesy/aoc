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

On Windows:

You're on your own.

### Day 1

#### Part One: Frequency Finder

This is on the face of it a simple little problem, almost more of a code kata
than anything else. It breaks down into three sections: taking input,
parsing that input for further use, and then doing something with that parsed
data.

One thing I learned here is how to do string indexing in Rust.

#### Part Two: Repeat as Needed

The Problem: Iterate through all the frequency changes, and find the first
instance when the frequency is repeated.

This project is a great example of the famous quote:

> "There is always a well-known solution to every human problem - neat, plausible,
> and wrong" -- HL Mencken, Prejudices: Second Series

The naive solution here is to just add every frequency (total) to a list and
check it every iteration of the loop - in pseudo code, something like:

```python
freqs = list()
total = 0

while True:
    get_co-ordinates
    for i in co-ordinates:
        # do math
        if total in list:
            print total
            break
        else:
            list.append(total)
```

However, this gets incredibly slow rather quickly, as you end up spending most
of your time in the 'if total in list' section, iterating over every past
answer. The better solution is to use a HashSet - instead of iterating over
every element of the list to see if there are duplicates, we can make ONE check.
This is due to the nature of a HashSet - instead of just  appending an element
to a list, we take the hash of the element, and use that hash to generate the
index of the element. Then, we check to see if that index is occupied. 

Think of this like looking through an apartment building to see if your friend
is home. You can go knock on each door one by one, checking if your friend is
there, or you can go directly to his door, and knock once. 

### Day 2
