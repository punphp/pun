<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\String;

if (!\function_exists(u::class)) {
    function u(?string $string = ''): UnicodeString
    {}
}

if (!\function_exists(b::class)) {
    function b(?string $string = ''): ByteString
    {}
}

if (!\function_exists(s::class)) {
    /**
     * @return UnicodeString|ByteString
     */
    function s(?string $string = ''): AbstractString
    {}
}
