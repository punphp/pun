<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Yaml;

use Symfony\Component\Yaml\Exception\ParseException;

/**
 * Unescaper encapsulates unescaping rules for single and double-quoted
 * YAML strings.
 *
 * @author Matthew Lewinski <matthew@lewinski.org>
 *
 * @internal
 */
class Unescaper
{
    /**
     * Regex fragment that matches an escaped character in a double quoted string.
     */
    public const REGEX_ESCAPED_CHARACTER = '\\\\(x[0-9a-fA-F]{2}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|.)';

    /**
     * Unescapes a single quoted string.
     *
     * @param string $value A single quoted string
     */
    public function unescapeSingleQuotedString(string $value): string
    {}

    /**
     * Unescapes a double quoted string.
     *
     * @param string $value A double quoted string
     */
    public function unescapeDoubleQuotedString(string $value): string
    {}

    /**
     * Unescapes a character that was found in a double-quoted string.
     *
     * @param string $value An escaped character
     */
    private function unescapeCharacter(string $value): string
    {}

    /**
     * Get the UTF-8 character for the given code point.
     */
    private static function utf8chr(int $c): string
    {}
}
