<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php82;

/**
 * @author Alexander M. Turek <me@derrabus.de>
 * @author Greg Roach <greg@subaqua.co.uk>
 *
 * @internal
 */
class Php82
{
    /**
     * Determines if a string matches the ODBC quoting rules.
     *
     * A valid quoted string begins with a '{', ends with a '}', and has no '}'
     * inside of the string that aren't repeated (as to be escaped).
     *
     * These rules are what .NET also follows.
     *
     * @see https://github.com/php/php-src/blob/838f6bffff6363a204a2597cbfbaad1d7ee3f2b6/main/php_odbc_utils.c#L31-L57
     */
    public static function odbc_connection_string_is_quoted(string $str): bool
    {}

    /**
     * Determines if a value for a connection string should be quoted.
     *
     * The ODBC specification mentions:
     * "Because of connection string and initialization file grammar, keywords and
     * attribute values that contain the characters []{}(),;?*=!@ not enclosed
     * with braces should be avoided."
     *
     * Note that it assumes that the string is *not* already quoted. You should
     * check beforehand.
     *
     * @see https://github.com/php/php-src/blob/838f6bffff6363a204a2597cbfbaad1d7ee3f2b6/main/php_odbc_utils.c#L59-L73
     */
    public static function odbc_connection_string_should_quote(string $str): bool
    {}

    public static function odbc_connection_string_quote(string $str): string
    {}

    /**
     * Implementation closely based on the original C code - including the GOTOs
     * and pointer-style string access.
     *
     * @see https://github.com/php/php-src/blob/master/Zend/zend_ini.c
     */
    public static function ini_parse_quantity(string $value): int
    {}

    /**
     * Escape the string to avoid null bytes and to make non-printable chars visible.
     */
    private static function escapeString(string $string): string
    {}
}
