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

## Day 1

### Part One: Frequency Finder

This is on the face of it a simple little problem, almost more of a code kata
than anything else. It breaks down into three sections: taking input,
parsing that input for further use, and then doing something with that parsed
data.

One thing I learned here is how to do string indexing in Rust.

### Part Two: Repeat as Needed

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

## Day 2

### Part 2

Problem: Given a list of random-ish strings, go through all of them and keep
two counts - one of all strings that have exactly 2 of any letter, and one of
all strings that have exactly 3 of any letter, then multiply those two values
together. 

Similarly to the last day's part 2, this problem becomes much easier with
hashes, though we use a HashMap here instead of a HashSet. The difference
between the two is pretty simple - with a set, the key and the value are the
same. A HashMap is a tuple `(key, value)`. Here, the key is going to be each
letter in the string, and the value will be the count of each time the letter
can be found in the string. 

At first I was a bit stumped about how to handle this - I knew I needed a
HashMap, and I knew it needed a count, but handling empty keys vs adding one to
the value of a key either gave me errors, or was ugly. Fortunately, the
`HashMap` struct implements the `Entry` API, which gives us the lovely
`or_insert()` method. `Entry` is an `enum` that wraps around `<key, value>` and keeps track of if an entry in a
map is `Vacant` or `Occupied`. `or_insert` returns a mutable reference to the
value at the key if it's `Occupied`, or the default value you specifed if the
entry is `Occupied`. 

The relevant section of the solution:

```rust
for i in data{
// set up  some variables
    let char_counter = HashMap::new();
    for ch in i.chars(){
        let counter = char_counter.entry(ch).or_insert(0);
        *counter +=1;
    }
// ...check the counter values and increment storage variables as needed
}
```

So, for every entry in data, we create an empty HashMap, then iterate over every
`char` in the entry. If the `char` is already in `char_counter`, we return a
reference to the value; if it's empty, we return 0. Finally, we add one to the
counter value. Once we've done that for every `char` in the entry, we check if
it satisfies our 2 or 3 instances condition, and then update our main counter
variables accordingly. 

One thing I'd like to change out this code is the way I check doubles and
triples per each string - it's just two if statements, which is...fine, but I
think it could be better done using a match. 
