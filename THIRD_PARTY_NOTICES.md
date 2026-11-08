# Third-party notices

Pun's own code is licensed under the MIT license in `LICENSE`. The union-type
sorting port includes the third-party code and notices below.

## Zend Engine sorting

`crates/pun-phpstan/src/types/sort.rs` ports sorting code from PHP 8.5.12,
revision `136e049ac10b032c83018e65b63b6df858943750`:
[Zend/zend_sort.c](https://github.com/php/php-src/blob/136e049ac10b032c83018e65b63b6df858943750/Zend/zend_sort.c).
The following license is reproduced from
[Zend/LICENSE](https://github.com/php/php-src/blob/136e049ac10b032c83018e65b63b6df858943750/Zend/LICENSE).

This product includes the Zend Engine, freely available at
http://www.zend.com

```text
--------------------------------------------------------------------
                The Zend Engine License, Version 2.00
Copyright (c) 1999-2006 Zend Technologies Ltd. All rights reserved.
--------------------------------------------------------------------

Redistribution and use in source and binary forms, with or without
modification, is permitted provided that the following conditions
are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.

  2. Redistributions in binary form must reproduce the above
     copyright notice, this list of conditions and the following
     disclaimer in the documentation and/or other materials provided
     with the distribution.

  3. The names "Zend" and "Zend Engine" must not be used to endorse
     or promote products derived from this software without prior
     permission from Zend Technologies Ltd. For written permission,
     please contact license@zend.com.

  4. Zend Technologies Ltd. may publish revised and/or new versions
     of the license from time to time. Each version will be given a
     distinguishing version number.
     Once covered code has been published under a particular version
     of the license, you may always continue to use it under the
     terms of that version. You may also choose to use such covered
     code under the terms of any subsequent version of the license
     published by Zend Technologies Ltd. No one other than Zend
     Technologies Ltd. has the right to modify the terms applicable
     to covered code created under this License.

  5. Redistributions of any form whatsoever must retain the following
     acknowledgment:
     "This product includes the Zend Engine, freely available at
     http://www.zend.com"

  6. All advertising materials mentioning features or use of this
     software must display the following acknowledgment:
     "The Zend Engine is freely available at http://www.zend.com"

THIS SOFTWARE IS PROVIDED BY ZEND TECHNOLOGIES LTD. ``AS IS'' AND
ANY EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL ZEND
TECHNOLOGIES LTD.  BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
SUCH DAMAGE.

--------------------------------------------------------------------
```

## LLVM libc++ sorting

The `zend_sort` implementation in the same upstream file derives from LLVM
libc++'s `std::sort`. Pun uses the MIT option of that code's dual license.
The following notice is reproduced from the MIT license block in
[Zend/zend_sort.c](https://github.com/php/php-src/blob/136e049ac10b032c83018e65b63b6df858943750/Zend/zend_sort.c#L227-L246).

```text
Copyright (c) 2009-2012 by the contributors listed at
http://llvm.org/svn/llvm-project/libcxx/trunk/CREDITS.TXT

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to
deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE.
```

## Mago

`third_party/mago-analyzer`, `third_party/mago-codex`, `third_party/mago-syntax` and
`third_party/mago-phpdoc-syntax` are copies of [Mago](https://github.com/carthage-software/mago) 1.57.0 crates
with Pun's patches (each crate's `PATCHES.md` lists them). They keep Mago's dual MIT and Apache-2.0 license; the
texts are in each crate's `LICENSE-MIT` and `LICENSE-APACHE`.
