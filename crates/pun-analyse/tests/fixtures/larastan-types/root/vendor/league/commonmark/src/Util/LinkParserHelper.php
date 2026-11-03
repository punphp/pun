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

use League\CommonMark\Parser\Cursor;

/**
 * @psalm-immutable
 */
final class LinkParserHelper
{
    /**
     * Attempt to parse link destination
     *
     * @return string|null The string, or null if no match
     */
    public static function parseLinkDestination(Cursor $cursor): ?string
    {}

    public static function parseLinkLabel(Cursor $cursor): int
    {}

    public static function parsePartialLinkLabel(Cursor $cursor): ?string
    {}

    /**
     * Attempt to parse link title (sans quotes)
     *
     * @return string|null The string, or null if no match
     */
    public static function parseLinkTitle(Cursor $cursor): ?string
    {}

    public static function parsePartialLinkTitle(Cursor $cursor, string $endDelimiter): ?string
    {}

    private static function manuallyParseLinkDestination(Cursor $cursor): ?string
    {}

    /** @var \WeakReference<Cursor>|null */
    private static ?\WeakReference $lastCursor       = null;
    private static bool $lastCursorLacksClosingBrace = false;

    private static function parseDestinationBraces(Cursor $cursor): ?string
    {}
}
