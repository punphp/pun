//! PHP's sorting path used by `UnionTypeHelper::sortTypes()`. Keep the comparison sequence: its
//! NullType comparator is non-reflexive, so a different stable sort does not produce the same order.
//!
//! Ported from php/php-src 136e049ac10b032c83018e65b63b6df858943750 (PHP 8.5.12):
//! - `Zend/zend_sort.c`: `zend_sort_2` through `zend_sort_5`, `zend_insert_sort`, `zend_sort`.
//! - `Zend/zend_hash.c`: `zend_hash_sort_internal` stores original bucket positions.
//! - `ext/standard/array.c`: `php_array_user_compare` / `stable_sort_fallback` break zero-result ties.

/*
 * The Zend Engine License, Version 2.00
 * Copyright (c) 1999-2006 Zend Technologies Ltd. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, is permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials provided
 *    with the distribution.
 * 3. The names "Zend" and "Zend Engine" must not be used to endorse
 *    or promote products derived from this software without prior
 *    permission from Zend Technologies Ltd. For written permission,
 *    please contact license@zend.com.
 * 4. Zend Technologies Ltd. may publish revised and/or new versions
 *    of the license from time to time. Each version will be given a
 *    distinguishing version number.
 *    Once covered code has been published under a particular version
 *    of the license, you may always continue to use it under the
 *    terms of that version. You may also choose to use such covered
 *    code under the terms of any subsequent version of the license
 *    published by Zend Technologies Ltd. No one other than Zend
 *    Technologies Ltd. has the right to modify the terms applicable
 *    to covered code created under this License.
 * 5. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes the Zend Engine, freely available at
 *    http://www.zend.com"
 * 6. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "The Zend Engine is freely available at http://www.zend.com"
 *
 * THIS SOFTWARE IS PROVIDED BY ZEND TECHNOLOGIES LTD. ``AS IS'' AND
 * ANY EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL ZEND
 * TECHNOLOGIES LTD. BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 * USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
 * OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

use std::cmp::Ordering;

use super::{Type, compare_in_union};

type Bucket<'a> = (usize, &'a Type);

pub(super) fn types(types: &[Type]) -> Vec<Type> {
    let mut buckets: Vec<_> = types.iter().enumerate().collect();
    zend_sort(&mut buckets);
    buckets.into_iter().map(|(_, t)| t.clone()).collect()
}

fn greater(buckets: &[Bucket<'_>], a: usize, b: usize) -> bool {
    compare_in_union(buckets[a].1, buckets[b].1).then_with(|| buckets[a].0.cmp(&buckets[b].0))
        == Ordering::Greater
}

fn zend_sort_2(buckets: &mut [Bucket<'_>], a: usize, b: usize) {
    if greater(buckets, a, b) {
        buckets.swap(a, b);
    }
}

fn zend_sort_3(buckets: &mut [Bucket<'_>], a: usize, b: usize, c: usize) {
    if !greater(buckets, a, b) {
        if !greater(buckets, b, c) {
            return;
        }
        buckets.swap(b, c);
        zend_sort_2(buckets, a, b);
        return;
    }
    if !greater(buckets, c, b) {
        buckets.swap(a, c);
        return;
    }
    buckets.swap(a, b);
    zend_sort_2(buckets, b, c);
}

fn zend_sort_4(buckets: &mut [Bucket<'_>], a: usize, b: usize, c: usize, d: usize) {
    zend_sort_3(buckets, a, b, c);
    if greater(buckets, c, d) {
        buckets.swap(c, d);
        if greater(buckets, b, c) {
            buckets.swap(b, c);
            zend_sort_2(buckets, a, b);
        }
    }
}

fn zend_sort_5(buckets: &mut [Bucket<'_>], a: usize, b: usize, c: usize, d: usize, e: usize) {
    zend_sort_4(buckets, a, b, c, d);
    if greater(buckets, d, e) {
        buckets.swap(d, e);
        if greater(buckets, c, d) {
            buckets.swap(c, d);
            if greater(buckets, b, c) {
                buckets.swap(b, c);
                zend_sort_2(buckets, a, b);
            }
        }
    }
}

fn zend_insert_sort(buckets: &mut [Bucket<'_>]) {
    match buckets.len() {
        0 | 1 => {}
        2 => zend_sort_2(buckets, 0, 1),
        3 => zend_sort_3(buckets, 0, 1, 2),
        4 => zend_sort_4(buckets, 0, 1, 2, 3),
        5 => zend_sort_5(buckets, 0, 1, 2, 3, 4),
        _ => {
            for i in 1..6 {
                let mut j = i - 1;
                if !greater(buckets, j, i) {
                    continue;
                }
                while j != 0 {
                    j -= 1;
                    if !greater(buckets, j, i) {
                        j += 1;
                        break;
                    }
                }
                for k in (j + 1..=i).rev() {
                    buckets.swap(k, k - 1);
                }
            }
            for i in 6..buckets.len() {
                let mut j = i - 1;
                if !greater(buckets, j, i) {
                    continue;
                }
                loop {
                    j -= 2;
                    if !greater(buckets, j, i) {
                        j += 1;
                        if !greater(buckets, j, i) {
                            j += 1;
                        }
                        break;
                    }
                    if j == 0 {
                        break;
                    }
                    if j == 1 {
                        j -= 1;
                        if greater(buckets, i, j) {
                            j += 1;
                        }
                        break;
                    }
                }
                for k in (j + 1..=i).rev() {
                    buckets.swap(k, k - 1);
                }
            }
        }
    }
}

/*
 * zend_sort is derived from LLVM libc++'s std::sort, under its MIT license:
 *
 * Copyright (c) 2009-2012 by the contributors listed at
 * http://llvm.org/svn/llvm-project/libcxx/trunk/CREDITS.TXT
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * WITH THE SOFTWARE.
 */
fn zend_sort(mut buckets: &mut [Bucket<'_>]) {
    loop {
        let len = buckets.len();
        if len <= 16 {
            zend_insert_sort(buckets);
            return;
        }
        let offset = len >> 1;
        if len >> 10 != 0 {
            let delta = offset >> 1;
            zend_sort_5(buckets, 0, delta, offset, offset + delta, len - 1);
        } else {
            zend_sort_3(buckets, 0, offset, len - 1);
        }
        buckets.swap(1, offset);
        let pivot = 1;
        let mut i = 2;
        let mut j = len - 1;
        'partition: loop {
            while greater(buckets, pivot, i) {
                i += 1;
                if i == j {
                    break 'partition;
                }
            }
            j -= 1;
            if j == i {
                break;
            }
            while greater(buckets, j, pivot) {
                j -= 1;
                if j == i {
                    break 'partition;
                }
            }
            buckets.swap(i, j);
            i += 1;
            if i == j {
                break;
            }
        }
        buckets.swap(pivot, i - 1);
        let (left, right) = buckets.split_at_mut(i);
        if i - 1 < len - i {
            zend_sort(&mut left[..i - 1]);
            buckets = right;
        } else {
            zend_sort(right);
            buckets = &mut left[..i - 1];
        }
    }
}
