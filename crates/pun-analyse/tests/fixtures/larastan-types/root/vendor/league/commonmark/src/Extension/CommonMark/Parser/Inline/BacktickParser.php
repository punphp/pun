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

namespace League\CommonMark\Extension\CommonMark\Parser\Inline;

use League\CommonMark\Extension\CommonMark\Node\Inline\Code;
use League\CommonMark\Node\Inline\Text;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\Inline\InlineParserInterface;
use League\CommonMark\Parser\Inline\InlineParserMatch;
use League\CommonMark\Parser\InlineParserContext;

final class BacktickParser implements InlineParserInterface
{
    /**
     * Max bound for backtick code span delimiters.
     *
     * @see https://github.com/commonmark/cmark/commit/8ed5c9d
     */
    private const MAX_BACKTICKS = 1000;

    /** @var \WeakReference<Cursor>|null */
    private ?\WeakReference $lastCursor = null;
    private bool $lastCursorScanned     = false;

    /** @var array<int, int> backtick count => position of known ender */
    private array $seenBackticks = [];

    public function getMatchDefinition(): InlineParserMatch
    {}

    public function parse(InlineParserContext $inlineContext): bool
    {}

    /**
     * Locates the matching closer for a backtick code span.
     *
     * Leverages some caching to avoid traversing the same cursor multiple times when
     * we've already seen all the potential backtick closers.
     *
     * @see https://github.com/commonmark/cmark/commit/8ed5c9d
     *
     * @param int    $openTickLength Number of backticks in the opening sequence
     * @param Cursor $cursor         Cursor to scan
     *
     * @return bool True if a matching closer was found, false otherwise
     */
    private function findMatchingTicks(int $openTickLength, Cursor $cursor): bool
    {}
}
