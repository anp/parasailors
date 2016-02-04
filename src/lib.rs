#![deny(missing_docs)]

// The MIT License (MIT)
// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! ## Background
//! [parasailors](https://github.com/dikaiosune/parasailors) is a set of Rust bindings to the [parasail](https://github.com/jeffdaily/parasail) pairwise sequence alignment library, which is written in C. `parasail` uses vectorized/SIMD versions of the [Smith-Waterman](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm) and [Needleman-Wunsch](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm) algorithms for pairwise sequence alignment. `parasail` also includes a vectorized [semi-global alignment algorithm](https://en.wikipedia.org/wiki/Sequence_alignment#Global_and_local_alignments) which provides a global alignment for a query sequence and a local alignment for a reference sequence (useful with NGS reads that need to be mapped against a chromosome, for example).
//!
//! **WARNING**: The bindings are currently in an immature state, and it's not recommended to use them for any published results or production systems without some independent verification of both the underlying algorithm implementations and this bindings library.
//!
//! In the interest of ease of use, this crate provides a much simpler interface than the original library. The original C library provides dozens (hundreds?) of functions to use for alignment. Even though they only implement 3 algorithms, they vary based on which SIMD ISA is used, the integer width for the underlying calculations, whether statistics of the alignment are calculated, whether rows or columns from the dynamic programming matrix are returned, etc. However, the library also provides automatic SIMD feature detection (to dynamically dispatch functions based on CPU architecture), and an overflow-detecting method for picking the correct integer width for calculations. These dispatching functions are what are currently called in `parasailors`.
//!
//! ## Usage
//!
//! Nearly all `parasail` functions create a "profile" for your alignment query as a first step. However, this is wasteful when you may need to reuse a query profile across multiple reference alignments, so there is a family of functions which take a pointer to a profile instead of a raw query sequence. All `parasailors` functionality uses explicitly created profiles to encourage efficient reuse:
//!
//! First, an exact matching example:
//!
//! ```
//! # use parasailors::*;
//! let query_sequence = b"AAAAAAAAAA";
//! let reference =      b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! ```
//!
//! We'll use an identity substitution matrix for scoring:
//!
//! ```
//! # use parasailors::*;
//! # let query_sequence = b"AAAAAAAAAA";
//! # let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! let identity_matrix = Matrix::new(MatrixType::Identity);
//! ```
//!
//! And construct a profile for querying the references:
//!
//! ```
//! # use parasailors::*;
//! # let query_sequence = b"AAAAAAAAAA";
//! # let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! # let identity_matrix = Matrix::new(MatrixType::Identity);
//! let profile = Profile::new(query_sequence, &identity_matrix);
//! ```
//!
//! And now we can perform several different alignments with the same profile:
//!
//! ```
//! # use parasailors::*;
//! # let query_sequence = b"AAAAAAAAAA";
//! # let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! # let identity_matrix = Matrix::new(MatrixType::Identity);
//! # let profile = Profile::new(query_sequence, &identity_matrix);
//! assert_eq!(10, local_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(10, semi_global_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(-30, global_alignment_score(&profile, reference, 1, 1));
//! ```
//!
//! And a non-matching alignment:
//!
//! ```
//! # use parasailors::*;
//! # let query_sequence = b"AAAAAAAAAA";
//! # let identity_matrix = Matrix::new(MatrixType::Identity);
//! # let profile = Profile::new(query_sequence, &identity_matrix);
//! let reference = b"CCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! assert_eq!(0, local_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(-1, semi_global_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(-40, global_alignment_score(&profile, reference, 1, 1));
//! ```
//!
//! Some more examples with differing query/reference relationships:
//!
//! ```
//! # use parasailors::*;
//! # let identity_matrix = Matrix::new(MatrixType::Identity);
//! // a normal NGS read length
//! let query = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! let profile = Profile::new(query, &identity_matrix);
//!
//! // these should be exact matches, with score of 50
//! let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
//! assert_eq!(50, local_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(50, semi_global_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(50, global_alignment_score(&profile, reference, 1, 1));
//!
//! // these should be inexact matches with 2 edits, with score of 48
//! let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTCCTTTTTTNNNNNNNNN";
//! assert_eq!(48, local_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(48, semi_global_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(48, global_alignment_score(&profile, reference, 1, 1));
//!
//! // a shorter reference -- shouldn't really align well
//! let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTT";
//! assert_eq!(35, local_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(35, semi_global_alignment_score(&profile, reference, 1, 1));
//! assert_eq!(2, global_alignment_score(&profile, reference, 1, 1));
//! ```

extern crate libc;
extern crate parasail_sys;

mod align;
mod matrix;
mod profile;

pub use align::*;
pub use matrix::*;
pub use profile::*;
