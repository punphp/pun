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

namespace League\CommonMark\Parser\Block;

use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\CursorState;

/**
 * Result object for continuing parsing of a block; see static methods for constructors.
 *
 * @psalm-immutable
 */
final class BlockContinue
{
    /** @psalm-readonly */
    private ?CursorState $cursorState = null;

    /** @psalm-readonly */
    private bool $finalize;

    private function __construct(?CursorState $cursorState = null, bool $finalize = false)
    {}

    public function getCursorState(): ?CursorState
    {}

    public function isFinalize(): bool
    {}

    /**
     * Signal that we cannot continue here
     *
     * @return null
     */
    public static function none(): ?self
    {}

    /**
     * Signal that we're continuing at the given position
     */
    public static function at(Cursor $cursor): self
    {}

    /**
     * Signal that we want to finalize and close the block
     */
    public static function finished(): self
    {}
}
