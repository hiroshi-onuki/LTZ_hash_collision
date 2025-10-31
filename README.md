# Collision Search in LTZ Hash

This project is a supporting code for the paper
["A collision attack on the LTZ hash function based on a conjecture on supersingular non-superspecial isogeny graphs of dimension 2"](https://eprint.iacr.org/2025/***).
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

We already generated files for all primes in $[7, 97]$.
These are available in the directory [fp2_invariants](./fp2_invariants).

After generating the files in the above directory,
you can check the conjecture by running the shell script
[count_Fp2types.sh](./count_Fp2types.sh).
This script requires Magma.
It takes several days to check all files.

## Collision Search
There are two algorithms to search for collisions in the LTZ hash function.
One is our proposed algorithm based on the conjecture in the paper,
and the other is a generic attack using the birthday paradox.

To run the proposed algorithm, use the following command:
```
cargo run collision <p>
```
where `<p>` is $1033$ or
a prime such that $p \equiv 2, 3 \pmod{5}$
and in $[7, 1013]$ and $[3523, 5153]$.
The output will be the two distinct messages deriving the same hash value.
It takes several days to find a collision for $p > 3000$.
So, we recommend you to use `--release` option to speed up the computation.

To run the generic attack, use the following command:
```
cargo run generic <p>
```
where `<p>` is a prime such that $p \equiv 2, 3 \pmod{5}$ and in $[7, 1013]$.
The output will be the two distinct messages deriving the same hash value.