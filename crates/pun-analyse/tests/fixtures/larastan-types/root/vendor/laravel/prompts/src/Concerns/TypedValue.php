<?php

namespace Laravel\Prompts\Concerns;

use IntlBreakIterator;
use Laravel\Prompts\Key;

trait TypedValue
{
    /**
     * The value that has been typed.
     */
    protected string $typedValue = '';

    /**
     * The position of the virtual cursor.
     */
    protected int $cursorPosition = 0;

    /**
     * Track the value as the user types.
     */
    protected function trackTypedValue(string $default = '', bool $submit = true, ?callable $ignore = null, bool $allowNewLine = false): void
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): string
    {}

    /**
     * Add a virtual cursor to the value and truncate if necessary.
     */
    protected function addCursor(string $value, int $cursorPosition, ?int $maxWidth = null): string
    {}

    /**
     * Get a truncated string with the specified width from the end.
     */
    private function trimWidthBackwards(string $string, int $start, int $width): string
    {}

    /**
     * Delete from the start of the current word (before cursor) through the cursor.
     */
    protected function deleteWordBackward(): void
    {}

    /**
     * Character offset of the word boundary immediately before the cursor (Intl + punctuation).
     * Punctuation (e.g. . - _) is treated as a word boundary so "word.word" deletes in two steps.
     */
    protected function findWordStartBeforeCursor(): int
    {}

    /**
     * Start (character offset) of the last run of letters/numbers in string (punctuation breaks words).
     */
    protected function findLastWordStartByLettersAndNumbers(string $before): int
    {}
}
