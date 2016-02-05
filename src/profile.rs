// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// This software may be modified and distributed under the terms of the MIT license.  See the
// LICENSE file for details.

use std::marker::PhantomData;
use std::ops::Deref;

use parasail_sys::{ParasailProfile, parasail_profile_create_sat, parasail_profile_free};
use matrix::Matrix;

/// A container for a parasail query profile. Can be reused to re-align the same sequence against multiple references.
///
/// # Unsafe
///
/// Don't mutate the struct. Doing so could cause memory leaks.
///
/// # Examples
///
/// ```
/// # use parasailors::*;
/// let query_sequence = b"AAATTAGACCGCAANGNNAAA";
/// let identity_matrix = Matrix::new(MatrixType::Identity);
/// let profile = Profile::new(query_sequence, &identity_matrix);
/// ```
pub struct Profile<'a> {
    sequence_lifetime: PhantomData<&'a [u8]>,
    internal_rep: *mut ParasailProfile,
}

#[doc(hidden)]
impl<'a> Drop for Profile<'a> {
    fn drop(&mut self) {
        unsafe {
            parasail_profile_free(self.internal_rep);
        }
    }
}

#[doc(hidden)]
impl<'a> Deref for Profile<'a> {
    type Target = *mut ParasailProfile;

    fn deref(&self) -> &(*mut ParasailProfile) {
        &self.internal_rep
    }
}

impl<'a> Profile<'a> {
    /// Creates a new profile container and ties its lifetime to the query sequence.
    pub fn new(query_seq: &'a [u8], matrix: &'a Matrix) -> Self {
        unsafe {
            // this struct now owns this pointer, and will free on drop
            let profile_ptr = parasail_profile_create_sat(query_seq.as_ptr(),
                                                          query_seq.len() as i32,
                                                          **matrix);

            // since the C struct has a pointer to our query sequence data
            // we'll also store a lifetime'd reference to the query to make sure we don't
            // deref the query after it's been dropped
            Profile {
                sequence_lifetime: PhantomData,
                internal_rep: profile_ptr,
            }
        }
    }
}
