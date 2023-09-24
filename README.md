# rust-functional-adts

This repository dives into the exploration and implementation of algebraic data types of functional languages like Haskell in Rust. Starting with Semigroups, it aims to span Monoids, Functors, and everything you'd expect. The main objective is to elucidate these mathematical concepts via idiomatic Rust code - and later to implement similar concepts in Cairo, a rust-based smart contract language on Starknet, as possible.

A generous exploratory grant by StarkWare has helped make this possible.

Learn more about StarkWare and their awesome Starknet tech, a permissionless layer 2 network leveraging smart contracts developed in the Cairo language, [here](https://starkware.co/).

## Highlights

- Initial implementation of the `Semigroup` trait for some popular Rust types.
- Designing with extensibility in mind, allowing for incorporation of many essential Rust types and, obviously, wrappers.
- Comprehensive unit tests to ensure correctness - extensive prop test to confirm laws coming soon.

## Getting Started

1. Clone the repository:
   ```bash
   git clone [repository-url]

2. Run the tests:
   ```bash
   cargo test --all