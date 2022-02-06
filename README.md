# Rhue
(pronounced /ro͞o/)

A simple Rust thue interpreter, with some twists -- written as an excuse to explore Rust 2021 while learning with the Rust book and Rust By Example.

### Usage

Unlike a typical Thue interpreter, Rhue uses two separate files.
One file contains the rule text (_.rhule_), and a second file contains the starting program text (_.rhue_).

An example can be found in `examples/binary_incrementer`.

Rhue can be run from the built executable, or via cargo, in the following fashion:

```sh
cargo run ./examples/binary_incrementer/ruleset.rhule ./examples/binary_incrementer/program.rhue
```

### Rule constructions

Like any other Thue interpreter, there are three basic rules you can leverage:
* **Replace** (`::=`): replace the occurence of the _lhs_ with the _rhs_.
* **Input** (`::=:::`): replace the occurrence of the _lhs_ with the user's input, with the _rhs_ being the prompt.
* **Print** (`::=~`): replace the occurrence of the _lhs_ with empty string, and display the message on the _rhs_.


