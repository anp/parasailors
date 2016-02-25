// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// This software may be modified and distributed under the terms of the MIT license.  See the
// LICENSE file for details.

use libc::c_int;

use parasail_sys::{parasail_result_free, parasail_nw_striped_profile_sat,
                   parasail_sg_striped_profile_sat, parasail_sw_striped_profile_sat,
                   parasail_sg_stats_striped_sat};
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

/// Stores statistics from an alignment.
pub struct AlignmentStats {
    /// The score according to the substitution matrix and gap penalty scheme used.
    pub score: i64,
    /// Number of exactly matching characters.
    pub num_matches: u64,
    /// Number of positively scoring character substitutions (this is the same as num_matches when used an identity matrix).
    pub num_positive_subs: u64,
    /// The length of the found alignment.
    pub align_length: usize,
    /// The starting index (0-based) of the alignment in the query (usually 0).
    pub query_end: usize,
    /// The starting index (0-based) of the alignment in the reference.
    pub ref_end: usize,
}

/// Provides statistics for semi-global pairwise alignment using a vectorized algorithm.
///
/// This results in a series of statistics, including a score that corresponds to a global alignment for the query sequence (i.e. the sequence in the `Profile`) and a local alignment for the reference sequence. This is particularly useful when checking for the presence of an NGS read in a much longer reference sequence. This behaves like a global alignment, except that gaps at the start or end of the reference sequence's alignment are ignored.
///
/// Other statistics include the number of matching characters, the number of positive substitutions, the length of the found alignment, and the starting point in both sequences where the alignment starts.
///
/// # Examples
///
/// ```
/// # use parasailors::*;
/// let identity_matrix = Matrix::new(MatrixType::Identity);
/// let query = b"AAAACCCCCCCCCCGGG";
///
/// let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
/// let stats = semi_global_alignment_stats(query, reference, 1, 1, &identity_matrix);
/// assert_eq!(17, stats.score);
/// assert_eq!(17, stats.num_matches);
/// assert_eq!(17, stats.num_positive_subs);
/// assert_eq!(17, stats.align_length);
/// assert_eq!(17, stats.query_end);
/// assert_eq!(23, stats.ref_end);
/// ```
pub fn semi_global_alignment_stats(query_sequence: &[u8],
                                   database_sequence: &[u8],
                                   open_cost: i32,
                                   gap_extend_cost: i32,
                                   substitution_matrix: &::matrix::Matrix)
                                   -> AlignmentStats {

    unsafe {
        let result = parasail_sg_stats_striped_sat(query_sequence.as_ptr(),
                                                   query_sequence.len() as c_int,
                                                   database_sequence.as_ptr(),
                                                   database_sequence.len() as c_int,
                                                   open_cost,
                                                   gap_extend_cost,
                                                   **substitution_matrix);

        let score = (*result).score as i64;
        let num_matches = (*result).matches as u64;
        let num_subs = (*result).similar as u64;
        let align_len = (*result).length as usize;

        // calculate start from end
        let query_end = (*result).end_query as usize + 1;
        let ref_end = (*result).end_ref as usize + 1;

        parasail_result_free(result);

        AlignmentStats {
            score: score,
            num_matches: num_matches,
            num_positive_subs: num_subs,
            align_length: align_len,
            query_end: query_end,
            ref_end: ref_end,
        }
    }
}

#[test]
fn test_semiglobal_stats() {
    use matrix::{Matrix, MatrixType};
    use std::str;
    let identity_matrix = Matrix::new(MatrixType::Identity);
    let query = b"AAAACCCCCCCCCCGGG";

    let reference = b"AAAAAAAAAACCCCCCCCCCGGGGGGGGGGTTTTTTTTTTTNNNNNNNNN";
    let stats = semi_global_alignment_stats(query, reference, 1, 1, &identity_matrix);
    assert_eq!(17, stats.score);
    assert_eq!(17, stats.num_matches);
    assert_eq!(17, stats.num_positive_subs);
    assert_eq!(17, stats.align_length);
    assert_eq!(17, stats.query_end);
    assert_eq!(23, stats.ref_end);

    assert_eq!(str::from_utf8(query).unwrap(),
               str::from_utf8(&query[stats.query_end - stats.align_length..stats.query_end])
                   .unwrap());
    assert_eq!(str::from_utf8(query).unwrap(),
               str::from_utf8(&reference[stats.ref_end - stats.align_length..stats.ref_end])
                   .unwrap());

    // these two test cases "borrowed" mutably from rust-bio
    let x = b"ACCGTGGAT";
    let y = b"AAAAACCGTTGAT";
    let ident_with_penalty = Matrix::new(MatrixType::IdentityWithPenalty);
    let alignment = semi_global_alignment_stats(x, y, 5, 1, &ident_with_penalty);
    assert_eq!(7, alignment.score);
    assert_eq!(13, alignment.ref_end);
    assert_eq!(9, alignment.query_end);

    let x = b"CCGGCA";
    let y = b"ACCGTTGACGC";
    let alignment = semi_global_alignment_stats(x, y, 5, 1, &ident_with_penalty);
    assert_eq!(1, alignment.score);
    assert_eq!(1, alignment.ref_end);
    assert_eq!(6, alignment.query_end);
}
