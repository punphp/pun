<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Original code based on the CommonMark JS reference parser (https://bitly.com/commonmark-js)
 *  - (c) John MacFarlane
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Util;

/**
 * @psalm-immutable
 */
final class Html5EntityDecoder
{
    /**
     * @psalm-pure
     */
    public static function decode(string $entity): string
    {}

    /**
     * @param mixed $number
     *
     * @psalm-pure
     */
    private static function fromDecimal($number): string
    {}

    /**
     * @psalm-pure
     */
    private static function fromHex(string $hexChars): string
    {}
}
