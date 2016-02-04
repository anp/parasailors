# parasailors

[parasailors](https://github.com/dikaiosune/parasailors) is a set of Rust bindings to the [parasail](https://github.com/jeffdaily/parasail) vectorized pairwise sequence alignment library. `parasail` provides vectorized/SIMD versions of the [Smith-Waterman](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm), [Needleman-Wunsch](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm), [semi-global alignment](https://en.wikipedia.org/wiki/Sequence_alignment#Global_and_local_alignments) algorithms for pairwise DNA/protein sequence alignment.

In the interest of ease of use, this crate provides a much simpler interface than the original library. The original C library provides dozens (hundreds?) of functions to use for alignment. Even though they only implement 3 algorithms, they vary based on which SIMD ISA is used, the integer width for the underlying calculations, whether statistics of the alignment are calculated, whether rows or columns from the dynamic programming matrix are returned, etc. However, the library also provides automatic SIMD feature detection (to dynamically dispatch functions based on CPU architecture), and an overflow-detecting method for picking the correct integer width for calculations. These dispatching functions are what are currently called in `parasailors`.

**WARNING**: The bindings are currently in an immature state, and it's not recommended to use them for any published results or production systems without some independent verification of both the underlying algorithm implementations and this crate. If you find something worrying, please open an issue :).

## Benchmarks

Coming soon.

## Usage

Cargo.toml:

```toml
[dependencies]
parasailors = "0.1"
```

See the documentation (below) for usage examples.

## Documentation and Examples

Either clone the repo and run:

```bash
cargo doc
```

or visit the [documentation hosted on GitHub Pages](https://dikaiosune.github.io/parasailors).

## Requirements

The sub-crate with the FFI bindings (`parasail_sys`) relies on `cmake >= 2.8.8` to build the bundled version of parasail. It's recommended to build on a system with a comparable SIMD instruction set to the target machine (ideally the target machine itself). The `parasail` C library generally does a good job of feature detection, but your compiler may balk at producing vector instructions for an architecture on which it's not running, potentially robbing some performance.

## Contributions, Questions, and Issues

Contributions are more than welcome, especially if you're more familiar than I am with C (which is likely) or bioinformatics (which is also likely). I encourage any bug reports or requests on GitHub Issues, and I'm happy to review any pull requests.

If you have questions that aren't related to a bug or requested feature, please [email me](adam.n.perry@gmail.com) or find me in the Rust IRC channels (usually `#rust` and `#rust-beginners`, nick is the same as GitHub username).

## License(s)

All of the Rust code here is licensed under the [MIT license](https://opensource.org/licenses/MIT). However, the code under `parasail_sys/parasail_c` is not mine to license and adheres to the [Batelle BSD-style license](https://github.com/jeffdaily/parasail/blob/master/README.md#license-battelle-bsd-style).

## Citations

If needed, please cite the original paper for [parasail](https://github.com/jeffdaily/parasail/):

Daily, Jeff. 2015. "Scalable Parallel Methods for Analyzing Metagenomic Data at Extreme Scale". PhD dissertation, Washington State University.  http://www.pnnl.gov/main/publications/external/technical_reports/PNNL-24266.pdf

I take no responsibility for the nice implementations of these algorithms, and I do think that the original author should be rewarded if this crate is used in an academic context.
