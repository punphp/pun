<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Parser;

use League\CommonMark\Exception\UnexpectedEncodingException;

class Cursor
{
    public const INDENT_LEVEL = 4;

    /**
     * Interval (in characters) between recorded byte-offset checkpoints on multibyte lines.
     * A larger interval uses less memory but makes random lookups walk further; a smaller one
     * costs more memory but speeds random lookups. Sequential access is O(1) regardless, via the
     * last-resolved-position cache, so this only trades memory against the cost of random jumps
     * (backwards peeks, saveState/restoreState).
     */
    private const BYTE_OFFSET_CHECKPOINT_INTERVAL = 16;

    /** @psalm-readonly */
    private string $line;

    /** @psalm-readonly */
    private int $length;

    /**
     * @var int
     *
     * It's possible for this to be 1 char past the end, meaning we've parsed all chars and have
     * reached the end.  In this state, any character-returning method MUST return null.
     */
    private int $currentPosition = 0;

    private int $column = 0;

    private int $indent = 0;

    private int $previousPosition = 0;

    private ?int $nextNonSpaceCache = null;

    private bool $partiallyConsumedTab = false;

    /**
     * @var int|false
     *
     * @psalm-readonly
     */
    private $lastTabPosition;

    /** @psalm-readonly */
    private bool $isMultibyte;

    /**
     * Sparse lookup table mapping a checkpoint index (character index divided by
     * BYTE_OFFSET_CHECKPOINT_INTERVAL) to the byte offset at which that character begins within
     * $line. Only populated for multibyte lines, and only ever extended forwards. Recording one
     * offset per interval - rather than one per character - bounds this map to O(n / interval)
     * memory, so it can never become the dominant allocation on a long line. A lookup walks at most
     * `interval` bytes from the nearest checkpoint (or O(1) from the last-resolved position for the
     * sequential access that dominates parsing); this is what keeps character<->byte translation -
     * and therefore every substring accessor - linear rather than O(n^2) on multibyte lines.
     *
     * @var array<int, int>
     */
    private array $byteOffsetCheckpoints = [];

    /** Character index of the most recently resolved byte offset (paired with $lastByteOffset). */
    private int $lastByteOffsetPosition = 0;

    /** Byte offset of the most recently resolved character index (paired with $lastByteOffsetPosition). */
    private int $lastByteOffset = 0;

    /**
     * Small cache of individually-read single characters on multibyte lines, keyed by
     * character index. Only populated for characters actually read one-at-a-time (e.g.
     * repeated peek() during delimiter scanning), so it stays tiny; substring accessors
     * bypass it entirely. Avoids re-slicing the same character on every lookup.
     *
     * @var array<int, string>
     */
    private array $charCache = [];

    /**
     * @param string $line The line being parsed (ASCII or UTF-8)
     */
    public function __construct(string $line)
    {}

    /**
     * Translate a character index into its byte offset within $line.
     *
     * The scan starts from the closest known reference point - the last-resolved position when it
     * sits between the target and the nearest earlier checkpoint (the common sequential case, O(1)),
     * otherwise that checkpoint (at most `interval` characters away). It then walks forwards one
     * character at a time, skipping UTF-8 continuation bytes (0b10xxxxxx) and recording a checkpoint
     * every `interval` characters. Sequential access is therefore amortized O(1) and any random
     * lookup is bounded by O(interval), all while the map itself stays O(n / interval) in memory.
     */
    private function byteOffset(int $index): int
    {}

    /**
     * Return the single multibyte character at $index, caching the sliced string so
     * repeated reads of the same position (common during delimiter scanning) don't
     * re-slice. Callers must ensure 0 <= $index < $length.
     *
     * Only call this when $isMultibyte is true. On a single-byte line the character is
     * already reachable as $this->line[$index], which costs less than the byte-offset
     * translation below; every caller guards on that flag for exactly that reason.
     */
    private function charAt(int $index): string
    {}

    /**
     * Returns the position of the next character which is not a space (or tab)
     */
    public function getNextNonSpacePosition(): int
    {}

    /**
     * Returns the next character which isn't a space (or tab)
     */
    public function getNextNonSpaceCharacter(): ?string
    {}

    /**
     * Calculates the current indent (number of spaces after current position)
     */
    public function getIndent(): int
    {}

    /**
     * Whether the cursor is indented to INDENT_LEVEL
     */
    public function isIndented(): bool
    {}

    public function getCharacter(?int $index = null): ?string
    {}

    /**
     * Slightly-optimized version of getCurrent(null)
     */
    public function getCurrentCharacter(): ?string
    {}

    /**
     * Returns the next character (or null, if none) without advancing forwards
     */
    public function peek(int $offset = 1): ?string
    {}

    /**
     * Whether the remainder is blank
     */
    public function isBlank(): bool
    {}

    /**
     * Move the cursor forwards
     */
    public function advance(): void
    {}

    /**
     * Move the cursor forwards
     *
     * @param int  $characters       Number of characters to advance by
     * @param bool $advanceByColumns Whether to advance by columns instead of spaces
     */
    public function advanceBy(int $characters, bool $advanceByColumns = false): void
    {}

    /**
     * Advances the cursor by a single space or tab, if present
     */
    public function advanceBySpaceOrTab(): bool
    {}

    /**
     * Parse zero or more space/tab characters
     *
     * @return int Number of positions moved
     */
    public function advanceToNextNonSpaceOrTab(): int
    {}

    /**
     * Parse zero or more space characters, including at most one newline.
     *
     * Tab characters are not parsed with this function.
     *
     * @return int Number of positions moved
     */
    public function advanceToNextNonSpaceOrNewline(): int
    {}

    /**
     * Move the position to the very end of the line
     *
     * @return int The number of characters moved
     */
    public function advanceToEnd(): int
    {}

    public function getRemainder(): string
    {}

    public function getLine(): string
    {}

    public function isAtEnd(): bool
    {}

    /**
     * Try to match a regular expression against the remainder of the line
     *
     * The subject begins at the cursor: text before the cursor is invisible to the pattern,
     * so "^" and "\A" anchor at the cursor, and constructs which examine what precedes the
     * match position (lookbehinds, "\b", "\B") see the start of a subject there rather than
     * the characters actually preceding the cursor.
     *
     * Returns the matching text and advances to the end of that match
     *
     * @psalm-param non-empty-string $regex
     */
    public function match(string $regex): ?string
    {}

    /**
     * Try to match a regular expression at the cursor's position within the line, without
     * copying the remainder
     *
     * Matches with PCRE's native offset semantics: the whole line is the subject, and matching
     * starts at the cursor. "\G" anchors at the cursor; "^" anchors at the true start of the
     * line (or after newlines under the "m" modifier); lookbehinds, "\b", and "\B" see the
     * characters actually preceding the cursor. This differs from match(), whose subject begins
     * at the cursor - a pattern written for match() migrates by replacing its leading "^" (or
     * "\A") with "\G".
     *
     * Because no copy of the remainder is made, repeated calls stay linear: match() copies
     * everything left in the line on every call, so scanning loops (such as the backtick closer
     * search) would otherwise cost O(n^2).
     *
     * When a tab has been partially consumed, no position within the line can represent the
     * cursor, so this falls back to matching the remainder with the leftover tab expanded into
     * spaces; "\G" still anchors at the cursor there, but the line content before it is not
     * visible in that case.
     *
     * @psalm-param non-empty-string $regex
     */
    public function matchInPlace(string $regex): ?string
    {}

    /**
     * Encapsulates the current state of this cursor in case you need to rollback later.
     *
     * WARNING: Do not parse or use the return value for ANYTHING except for
     * passing it back into restoreState(), as the number of values and their
     * contents may change in any future release without warning.
     */
    public function saveState(): CursorState
    {}

    /**
     * Restore the cursor to a previous state.
     *
     * Pass in the value previously obtained by calling saveState().
     */
    public function restoreState(CursorState $state): void
    {}

    public function getPosition(): int
    {}

    /**
     * Returns the byte offset of the current position within the line.
     *
     * For single-byte lines this is identical to getPosition(). For multibyte
     * lines the offset comes from the lazily-built character->byte map, which
     * makes this an amortized-O(1) lookup rather than O(position) per call.
     */
    public function getBytePosition(): int
    {}

    public function getPreviousText(): string
    {}

    public function getSubstring(int $start, ?int $length = null): string
    {}

    public function getColumn(): int
    {}
}
