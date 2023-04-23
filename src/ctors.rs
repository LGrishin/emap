// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{Item, Map};
use std::mem::MaybeUninit;

impl<V: Clone + Copy, const N: usize> Default for Map<V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Copy, const N: usize> Map<V, N> {
    /// Make it.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            Self {
                filled: 0,
                items: *MaybeUninit::<[Item<V>; N]>::uninit().assume_init_mut(),
            }
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn makes_new_map() -> Result<()> {
    let m: Map<&str, 8> = Map::new();
    assert_eq!(0, m.len());
    Ok(())
}
