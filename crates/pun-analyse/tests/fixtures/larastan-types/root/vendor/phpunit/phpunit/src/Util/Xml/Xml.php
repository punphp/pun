<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util;

use const ENT_QUOTES;
use function assert;
use function htmlspecialchars;
use function mb_convert_encoding;
use function ord;
use function preg_replace;
use function strlen;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Xml
{
    /**
     * Escapes a string for the use in XML documents.
     *
     * Any Unicode character is allowed, excluding the surrogate blocks, FFFE,
     * and FFFF (not even as character reference).
     *
     * @see https://www.w3.org/TR/xml/#charsets
     */
    public static function prepareString(string $string): string
    {}

    private static function convertToUtf8(string $string): string
    {}

    private static function isUtf8(string $string): bool
    {}
}
