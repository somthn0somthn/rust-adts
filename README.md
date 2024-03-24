# rust-functional-adts

This repository dives into the emulation of algebraic data types of functional languages like Haskell in Rust. The basic trait type is taken from this amazing project by [edmundsmith](https://gist.github.com/edmundsmith/855fcf0cb35dd467c29a9350481f0ecf). This project's clever plug + unplug method is built upon to explore basic functional programming design types that are hopefully applicable across Rust-based DSLs, including the modern Starknet Cairo language. This repository represents an initial Rust-implementation upon which a Cairo version may be based.

A generous exploratory grant by StarkWare has helped make this possible.

Learn more about StarkWare and their awesome Starknet tech, a permissionless layer 2 network leveraging smart contracts developed in the Cairo language, [here](https://starkware.co/).

## Highlights

- Initial implementation of the `Monoid` and `Functo` traits for some popular Rust types.
- Designing with extensibility in mind, allowing for incorporation of many essential Rust types and, obviously, wrappers.
- Comprehensive unit tests to ensure correctness - extensive prop test to confirm laws coming soon.

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/somthn0somthn/rust-functional-adts.git

2. Run the tests:
   ```bash
   cargo test --all
