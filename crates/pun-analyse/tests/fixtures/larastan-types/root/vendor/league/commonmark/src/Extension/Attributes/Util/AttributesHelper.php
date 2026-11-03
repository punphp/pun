<?php

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 * (c) 2015 Martin Hasoň <martin.hason@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\CommonMark\Extension\Attributes\Util;

use League\CommonMark\Node\Node;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Util\RegexHelper;

/**
 * @internal
 */
final class AttributesHelper
{
    private const SINGLE_ATTRIBUTE = '\s*([.]-?[_a-z][^\s.}]*|[#][^\s}]+|' . RegexHelper::PARTIAL_ATTRIBUTENAME . RegexHelper::PARTIAL_ATTRIBUTEVALUESPEC . ')\s*';
    private const ATTRIBUTE_LIST   = '/\G{:?(' . self::SINGLE_ATTRIBUTE . ')+}/i';

    /**
     * PCRE's `\s` matches the form feed that PHP's default trim charlist omits, so the
     * separators SINGLE_ATTRIBUTE accepts must be trimmed with this list instead - otherwise
     * that byte survives inside an attribute name, where a browser reads it as a separator.
     */
    private const WHITESPACE = " \t\n\r\0\x0B\x0C";

    /**
     * @return array<string, mixed>
     */
    public static function parseAttributes(Cursor $cursor): array
    {}

    /**
     * @param Node|array<string, mixed> $attributes1
     * @param Node|array<string, mixed> $attributes2
     *
     * @return array<string, mixed>
     */
    public static function mergeAttributes($attributes1, $attributes2): array
    {}

    /**
     * Split a `class` attribute value into the individual classes it contributes to a merge
     *
     * @param mixed $class
     *
     * @return list<string>
     */
    public static function classList($class): array
    {}

    /**
     * @param array<string, mixed> $attributes
     * @param list<string>         $allowList
     *
     * @return array<string, mixed>
     */
    public static function filterAttributes(array $attributes, array $allowList, bool $allowUnsafeLinks): array
    {}
}
