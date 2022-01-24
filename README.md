# ripgen
----

`ripgen` is based on the popular [dnsgen](https://github.com/ProjectAnte/dnsgen) python utility.

`ripgen` is split into two main parts:

* **ripgen**: _A CLI utility that calls into `ripgen_lib` and uses dnsgen's transforms.
* **ripgen_lib**: A library that allows you to create high performance permutations of domain names.

# How to Install!
Installation of `ripgen` is very simple - follow the steps below.

### Step 1 - rustup.rs
Visit https://rustup.rs/ and follow the instructions to get started with `rust` and `cargo`.

### Step 2 - cargo install
Run `cargo install ripgen`

## How to Use - `ripgen`
`ripgen` optionally takes a domain file, a wordlist file, and a minimum word length argument.

If no domain file is listed, domains are expected through `stdin` making it easy to pipe into from other tools.

e.g.
```
$ echo "www1.google.com" | ripgen
```

One deviation from dnsgen's behavior is that if no wordlist is specified then no wordlist items are included automatically. To compare `ripgen` and `dnsgen` appropriately you should make sure to specify a wordlist.

## How to use - `ripgen_lib`
`ripgen_lib` exposes a `RipGenManager` struct that takes in three components:

* an iterator for domain names
* an iterator for wordlist entries
* a function that converts `&&str` into `bool` for the purposes of filtering wordlist entries

After creating a `RipGenManager`, transforms can be added on with `transform` and `chain_transform`. These transforms require a function definition (closure or otherwise) be passed in that can take the `&DomainComponent` and `WordListIterator` types and return an `Iterator<Item = String>`.

Look at the non-default dnsgen transform implementations for examples on how these are implemented typically.