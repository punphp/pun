<?php

namespace Laravel\Prompts\Concerns;

trait Cursor
{
    /**
     * Indicates if the cursor has been hidden.
     */
    protected static bool $cursorHidden = false;

    /**
     * Hide the cursor.
     */
    public function hideCursor(): void
    {}

    /**
     * Show the cursor.
     */
    public function showCursor(): void
    {}

    /**
     * Restore the cursor if it was hidden.
     */
    public function restoreCursor(): void
    {}

    /**
     * Move the cursor.
     */
    public function moveCursor(int $x, int $y = 0): void
    {}

    /**
     * Move the cursor to the given column.
     */
    public function moveCursorToColumn(int $column): void
    {}

    /**
     * Move the cursor up by the given number of lines.
     */
    public function moveCursorUp(int $lines): void
    {}
}
