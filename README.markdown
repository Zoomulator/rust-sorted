# Sorted
Distributed under the [MIT license](./LICENSE).

[Documentation](https://zoomulator.github.io/rust-sorted/sorted/)

## What
A Rust library that provides compile time contracts for sorted sequences.

## Status
[![Latest version on crates.io](https://img.shields.io/crates/v/sorted.svg)](https://crates.io/crates/sorted)
[![Build Status](https://travis-ci.org/Zoomulator/rust-sorted.svg?branch=master)](https://travis-ci.org/Zoomulator/rust-sorted)

This library is currently being drafted and is not fit for any use at this time.

## Reasoning
By limiting sorted types to operations that keeps the sorting order it's
possible to give guarantees at compile time.
Even different kinds of sort ordering can be typed and forbids collections
sorted by different keys or comparisons to be mixed.

## Why
If you're dealing with a lot of sorted and unsorted arrays, vectors or
iterators in a lot of places in your code it can be easy to slip up and add
an element in the wrong place or use an unsorted list where a sorted one is
required. Compile time guarantees helps you avoid bugs and declare intentions
in those cases.

That said, this library won't provide any actual functionality to your code,
except a few convenience methods that keeps the sorted contract. If you're not
holding array types sorted for any length of time or bigger amounts, this
library is probably overkill.

## How
This library implements a number of constraint type wrappers that restricts the
mutable operations of Vec and slices to operations that retains the sorted order.
Non-mut operations are still provided via Deref.
These sorted types can only be created by sorting a regular sequence or copying
another sorted type.

# Examples
Look at [tests/usage.rs](./tests/usage.rs) on how to use the sorted types.
