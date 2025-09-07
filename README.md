# Collision Search in LTZ Hash

This project is a supporting code for the paper
["A Memory-Efficient Collision Attack on LeGrow-Ti-Zobernig's Hash Function"](https://eprint.iacr.org/2025/***).
This contains a code to confirm the conjecture in the paper and
an implementation of the collision search algorithm proposed in the paper.

## Confirmation of the Conjecture
We can confirm the conjecture in the paper for primes in $[7, 97]$.
To do this for a specific prime, run the following command:
```
cargo run count <p> <file>
```
where `<p>` is the prime and `<file>` is the output file name.
The output file contains Magma code to generate the corresponding Igusa invariants over $\mathbb{F}_{p^2}$.
If you want to generate files for all primes in $[7, 97]$, run the following command:
```
cargo run count all <dir>
```
where `<dir>` is the output directory.
Note that it takes several days to generate all files.
So, we recommend you to use `--release` option to speed up the computation.

After generating the file, you can check the conjecture by running the shell script
[count_FP2types.sh](./count_FP2types.sh).
This script requires Magma.
It takes a few days to check all files.

## Collision Search
To search for collisions, run the following command:
```
cargo run collision <p>
```
where `<p>` is $103, 1013, 1033$ or
a prime such that $p \equiv 2, 3 \pmod{5}$
and in $[3523, 5153]$.
The output will be the two distinct messages deriving the same hash value.
It takes several days to find a collision for $p > 3000$.
So, we recommend you to use `--release` option to speed up the computation.