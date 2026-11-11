# Third-party notices

Pun's own code is licensed under the MIT license in `LICENSE`. Pun also ports code, links libraries and copies test
data from the projects below; their notices follow. The Rust crates in the `pun` binary and their license texts are in
`THIRD_PARTY_LICENSES.md`.

## Zend Engine

`crates/pun-format/src/php_grammar.rs` holds parser tables generated from PHP 8.5.11's
`Zend/zend_language_parser.y`, and `crates/pun-phpstan/src/types/sort.rs` ports sorting code from PHP 8.5.12,
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
with Pun's patches (each crate's `PATCHES.md` lists them). `crates/pun-lint/tests/mago` holds copies of Mago's linter
tests. They keep Mago's dual MIT and Apache-2.0 license; the texts are in `THIRD_PARTY_LICENSES.md` (and in each
vendored crate's `LICENSE-MIT` and `LICENSE-APACHE`).

Mago's `mago-prelude` crate, which is in the binary, embeds PHP stubs modified from
[JetBrains/phpstorm-stubs](https://github.com/JetBrains/phpstorm-stubs), and `crates/pun-phpstan/data/symbols.json`
is exported from the copy of those stubs in PHPStan: Copyright 2010-2023 JetBrains s.r.o., Apache License 2.0 (the
text is in `THIRD_PARTY_LICENSES.md`).

## PCRE2

The binary links [PCRE2](https://github.com/PCRE2Project/pcre2) 10.46 statically (the copy in the `pcre2-sys` crate),
with its just-in-time compiler.

```text
PCRE2 License
=============

| SPDX-License-Identifier: | BSD-3-Clause WITH PCRE2-exception |
|---------|-------|

PCRE2 is a library of functions to support regular expressions whose syntax
and semantics are as close as possible to those of the Perl 5 language.

Releases 10.00 and above of PCRE2 are distributed under the terms of the "BSD"
licence, as specified below, with one exemption for certain binary
redistributions. The documentation for PCRE2, supplied in the "doc" directory,
is distributed under the same terms as the software itself. The data in the
testdata directory is not copyrighted and is in the public domain.

The basic library functions are written in C and are freestanding. Also
included in the distribution is a just-in-time compiler that can be used to
optimize pattern matching. This is an optional feature that can be omitted when
the library is built.


COPYRIGHT
---------

### The basic library functions

    Written by:       Philip Hazel
    Email local part: Philip.Hazel
    Email domain:     gmail.com

    Retired from University of Cambridge Computing Service,
    Cambridge, England.

    Copyright (c) 1997-2007 University of Cambridge
    Copyright (c) 2007-2024 Philip Hazel
    All rights reserved.

### PCRE2 Just-In-Time compilation support

    Written by:       Zoltan Herczeg
    Email local part: hzmester
    Email domain:     freemail.hu

    Copyright (c) 2010-2024 Zoltan Herczeg
    All rights reserved.

### Stack-less Just-In-Time compiler

    Written by:       Zoltan Herczeg
    Email local part: hzmester
    Email domain:     freemail.hu

    Copyright (c) 2009-2024 Zoltan Herczeg
    All rights reserved.

### All other contributions

Many other contributors have participated in the authorship of PCRE2. As PCRE2
has never required a Contributor Licensing Agreement, or other copyright
assignment agreement, all contributions have copyright retained by each
original contributor or their employer.


THE "BSD" LICENCE
-----------------

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notices,
  this list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright
  notices, this list of conditions and the following disclaimer in the
  documentation and/or other materials provided with the distribution.

* Neither the name of the University of Cambridge nor the names of any
  contributors may be used to endorse or promote products derived from this
  software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.


EXEMPTION FOR BINARY LIBRARY-LIKE PACKAGES
------------------------------------------

The second condition in the BSD licence (covering binary redistributions) does
not apply all the way down a chain of software. If binary package A includes
PCRE2, it must respect the condition, but if package B is software that
includes package A, the condition is not imposed on package B unless it uses
PCRE2 independently.

End
```

## Ported PHP tools

Pun's source ports the behaviour of the projects below, and the repository's tests hold copies of their test data.

| Project | Ported into | Copied test data |
| --- | --- | --- |
| [PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer) 3.95.25 | `crates/pun-tokens`, `crates/pun-format`, `crates/pun-fixers` | `crates/pun-fixers/tests/fixtures` |
| [Laravel Pint](https://github.com/laravel/pint) 1.32 | `crates/pun-fixers`, `crates/pun-config/presets`, `crates/pun-workspace`, `crates/pun-cli` | `crates/pun-fixers/tests/cases` (its output) |
| [PHPStan](https://github.com/phpstan/phpstan-src) 2.3.5 | `crates/pun-phpstan`, `crates/pun-analyse` | `crates/pun-analyse/tests/fixtures/rules`, `levels` and `nsrt`, `crates/pun-phpstan/tests/fixtures` |
| [Larastan](https://github.com/larastan/larastan) 3.14.0 | `crates/pun-analyse` | `crates/pun-analyse/tests/fixtures/larastan` and `larastan-types` |
| [Rector](https://github.com/rectorphp/rector-src) 2.8.4, with [rector-phpunit](https://github.com/rectorphp/rector-phpunit) | `crates/pun-lint` | `crates/pun-lint/tests/rector`, `rector-phpunit-stubs`, `doctrine-stubs` |
| [rector-laravel](https://github.com/driftingly/rector-laravel) 2.7.3 | `crates/pun-lint` | `crates/pun-lint/tests/rector`, `rector-laravel-stubs` |
| [pest-plugin-rector](https://github.com/pestphp/pest-plugin-rector) | `crates/pun-lint` | `crates/pun-lint/tests/rector` |
| [nette/neon, nette/schema, nette/di](https://github.com/nette) | `crates/pun-phpstan` | `crates/pun-phpstan/tests/fixtures` |
| [sebastian/diff](https://github.com/sebastianbergmann/diff) | `crates/pun-cli` (`php_diff`) | none |
| [PHPUnit](https://github.com/sebastianbergmann/phpunit) | none | `crates/pun-lint/tests/phpunit-stubs` |
| [nikic/php-parser](https://github.com/nikic/PHP-Parser) | none | `crates/pun-lint/tests/php-parser-stubs` |

`crates/pun-analyse/tests/fixtures/larastan-types/root/vendor` holds trimmed copies of the Composer packages
Larastan's tests load. Each package directory there carries its own license file.

### MIT license

PHP-CS-Fixer, Laravel Pint, PHPStan, Larastan, Rector, rector-phpunit, rector-laravel and pest-plugin-rector are under
the MIT license, with these copyright notices:

```text
PHP-CS-Fixer:        Copyright (c) 2012+ Fabien Potencier, Dariusz Rumiński
Laravel Pint:        Copyright (c) Taylor Otwell
PHPStan:             Copyright (c) 2016 Ondřej Mirtes
                     Copyright (c) 2025 PHPStan s.r.o.
Larastan:            Copyright (c) Can Vural <can9119@gmail.com>, Nuno Maduro <enunomaduro@gmail.com>,
Rector:              Copyright (c) 2017-present Tomas Votruba (https://tomasvotruba.com)
rector-phpunit:      Copyright (c) 2017-present Tomáš Votruba (https://tomasvotruba.cz)
rector-laravel:      Copyright (c) 2017-present Tomáš Votruba (https://tomasvotruba.cz)
pest-plugin-rector:  Copyright (c) Pest and contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Nette (BSD 3-Clause)

```text
Licenses
========

Good news! You may use Nette Framework under the terms of either
the New BSD License or the GNU General Public License (GPL) version 2 or 3.

The BSD License is recommended for most projects. It is easy to understand and it
places almost no restrictions on what you can do with the framework. If the GPL
fits better to your project, you can use the framework under this license.

You don't have to notify anyone which license you are using. You can freely
use Nette Framework in commercial projects as long as the copyright header
remains intact.

Please be advised that the name "Nette Framework" is a protected trademark and its
usage has some limitations. So please do not use word "Nette" in the name of your
project or top-level domain, and choose a name that stands on its own merits.
If your stuff is good, it will not take long to establish a reputation for yourselves.


New BSD License
---------------

Copyright (c) 2004, 2014 David Grudl (https://davidgrudl.com)
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

	* Redistributions of source code must retain the above copyright notice,
	this list of conditions and the following disclaimer.

	* Redistributions in binary form must reproduce the above copyright notice,
	this list of conditions and the following disclaimer in the documentation
	and/or other materials provided with the distribution.

	* Neither the name of "Nette Framework" nor the names of its contributors
	may be used to endorse or promote products derived from this software
	without specific prior written permission.

This software is provided by the copyright holders and contributors "as is" and
any express or implied warranties, including, but not limited to, the implied
warranties of merchantability and fitness for a particular purpose are
disclaimed. In no event shall the copyright owner or contributors be liable for
any direct, indirect, incidental, special, exemplary, or consequential damages
(including, but not limited to, procurement of substitute goods or services;
loss of use, data, or profits; or business interruption) however caused and on
any theory of liability, whether in contract, strict liability, or tort
(including negligence or otherwise) arising in any way out of the use of this
software, even if advised of the possibility of such damage.


GNU General Public License
--------------------------

GPL licenses are very very long, so instead of including them here we offer
you URLs with full text:

- [GPL version 2](https://www.gnu.org/licenses/gpl-2.0.html)
- [GPL version 3](https://www.gnu.org/licenses/gpl-3.0.html)
```

### sebastian/diff (BSD 3-Clause)

```text
BSD 3-Clause License

Copyright (c) 2002-2026, Sebastian Bergmann
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

### PHPUnit (BSD 3-Clause)

```text
BSD 3-Clause License

Copyright (c) 2001-2026, Sebastian Bergmann
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

### nikic/php-parser (BSD 3-Clause)

```text
BSD 3-Clause License

Copyright (c) 2011, Nikita Popov
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```
