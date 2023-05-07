# Rust Experiment

## Problem Statement

Given a project of the style:

```
|- src
|  |- main.rs
|  |- people.rs
|  |  |- bill.rs
|  |  |- john.rs
|  |  |- sarah.rs
```

where `bill`, `john` and `sarah` all implement the trait `Person`:

```rs
pub trait Person {
    fn greet(&self);
}

impl Person for Bill {
    fn greet(&self) {
        println!("Bill")
    }
}
//...
```

Can we automatically generate and iterate over a collection of all 'people' in the people directory of the project?

[related stackoverflow question](https://stackoverflow.com/questions/76191028/how-to-add-all-modules-to-array-vector?noredirect=1#comment134363819_76191028)

## Tests and solutions

| Branch                                                                                                               | Status  |
|----------------------------------------------------------------------------------------------------------------------|---------|
| [Not autonymous](https://github.com/sancarn/rust_experiment-distributed_handlers/tree/manual-declaration)            | N/A     |
| [Initial attempt using `ctor`](https://github.com/sancarn/rust_experiment-distributed_handlers/tree/initial-attempt) | Failed  |
| [Using `build.rs`](https://github.com/sancarn/rust_experiment-distributed_handlers/tree/using-build-process)         | Pass    |

After testing numerous methods I have only so far been able to get one method working fully autonymously, which is the `build.rs` method. `ctor` doesn't seem to work as it's difficult to create a global mutable list which is empty at compile time (maybe doable but I can't figure out how...). Even if it did work though, you still need to list all the modules with `mod bill; mod john; ...`.

Current best solution therefore is to use `build.rs` script. Current implementation assumes module name is lower case of struct name. A better approach would be to search in files for regex `impl Person for (\w+)` and add respectively.