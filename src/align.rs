// The MIT License (MIT)
// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use libc::c_int;

use parasail_sys::{parasail_nw_striped_profile_sat, parasail_result_free,
                   parasail_sg_striped_profile_sat, parasail_sw_striped_profile_sat};
use profile::Profile;

/// Provides a score for global pairwise alignment, using a vectorized version of [Needleman-Wunsch](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm).
///
/// # Examples
///
/// ```
/// # use parasailors::*;
/// // create & lookup substitution matrices
/// let identity_matrix = Matrix::new(MatrixType::Identity);
///
/// let query = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// let profile_ident = Profile::new(query, &identity_matrix);
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// assert_eq!(50, global_alignment_score(&profile_ident, reference, 1, 1));
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTCCTTTTTTNNNNNNNNN";
/// assert_eq!(48, global_alignment_score(&profile_ident, reference, 1, 1));
/// ```
pub fn global_alignment_score(query_profile: &Profile,
                              database_sequence: &[u8],
                              open_cost: i32,
                              gap_extend_cost: i32)
                              -> i32 {

    unsafe {
        let result = parasail_nw_striped_profile_sat(**query_profile,
                                                     database_sequence.as_ptr(),
                                                     database_sequence.len() as c_int,
                                                     open_cost,
                                                     gap_extend_cost);
        let score = (*result).score;
        parasail_result_free(result);
        score
    }
}

/// Provides a score for semi-global pairwise alignment using a vectorized algorithm.
///
/// This results in a score that corresponds to a global alignment for the query sequence (i.e. the sequence in the `Profile`) and a local alignment for the reference sequence. This is particularly useful when checking for the presence of an NGS read in a much longer reference sequence. This behaves like a global alignment, except that gaps at the start or end of the reference sequence's alignment are ignored.
///
/// # Examples
///
/// ```
/// # use parasailors::*;
/// # let identity_matrix = Matrix::new(MatrixType::Identity);
/// let query = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// let profile_ident = Profile::new(query, &identity_matrix);
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// assert_eq!(50, semi_global_alignment_score(&profile_ident, reference, 1, 1));
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTCCTTTTTTNNNNNNNNN";
/// assert_eq!(48, semi_global_alignment_score(&profile_ident, reference, 1, 1));
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTT";
/// assert_eq!(35, semi_global_alignment_score(&profile_ident, reference, 1, 1));
/// ```
pub fn semi_global_alignment_score(query_profile: &Profile,
                                   database_sequence: &[u8],
                                   open_cost: i32,
                                   gap_extend_cost: i32)
                                   -> i32 {

    unsafe {
        let result = parasail_sg_striped_profile_sat(**query_profile,
                                                     database_sequence.as_ptr(),
                                                     database_sequence.len() as c_int,
                                                     open_cost,
                                                     gap_extend_cost);
        let score = (*result).score;
        parasail_result_free(result);
        score
    }
}

/// Returns a score for local pairwise alignment using a vectorized version of [Smith-Waterman](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm).
///
/// # Examples
///
/// ```
/// # use parasailors::*;
/// # let identity_matrix = Matrix::new(MatrixType::Identity);
/// let query = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// let profile_ident = Profile::new(query, &identity_matrix);
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// assert_eq!(50, local_alignment_score(&profile_ident, reference, 1, 1));
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTCCTTTTTTNNNNNNNNN";
/// assert_eq!(48, local_alignment_score(&profile_ident, reference, 1, 1));
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTT";
/// assert_eq!(35, local_alignment_score(&profile_ident, reference, 1, 1));
/// ```
pub fn local_alignment_score(query_profile: &Profile,
                             database_sequence: &[u8],
                             open_cost: i32,
                             gap_extend_cost: i32)
                             -> i32 {

    unsafe {
        let result = parasail_sw_striped_profile_sat(**query_profile,
                                                     database_sequence.as_ptr(),
                                                     database_sequence.len() as c_int,
                                                     open_cost,
                                                     gap_extend_cost);
        let score = (*result).score;
        parasail_result_free(result);
        score
    }
}
