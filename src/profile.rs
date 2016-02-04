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
