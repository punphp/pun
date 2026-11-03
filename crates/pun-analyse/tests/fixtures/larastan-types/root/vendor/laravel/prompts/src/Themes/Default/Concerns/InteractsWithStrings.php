<?php

namespace Laravel\Prompts\Themes\Default\Concerns;

trait InteractsWithStrings
{
    /**
     * Get the length of the longest line.
     *
     * @param  array<string>  $lines
     */
    protected function longest(array $lines, int $padding = 0): int
    {}

    /**
     * Pad text ignoring ANSI escape sequences.
     */
    protected function pad(string $text, int $length, string $char = ' '): string
    {}

    /**
     * Strip ANSI escape sequences from the given text.
     */
    protected function stripEscapeSequences(string $text): string
    {}

    /**
     * Multi-byte version of wordwrap.
     *
     * @param  non-empty-string  $break
     */
    protected function mbWordwrap(
        string $string,
        int $width = 75,
        string $break = "\n",
        bool $cut_long_words = false
    ): string {}

    /**
     * Word wrap text while preserving ANSI escape sequences.
     *
     * @return array<int, string>
     */
    protected function ansiWordwrap(string $text, int $width): array
    {}

    /**
     * Parse text into segments with their associated ANSI codes.
     *
     * @return array<int, array{text: string, codes: string, link: string}>
     */
    protected function parseAnsiText(string $text): array
    {}
}
