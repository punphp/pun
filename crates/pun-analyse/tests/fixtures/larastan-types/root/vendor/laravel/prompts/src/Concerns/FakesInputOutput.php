<?php

namespace Laravel\Prompts\Concerns;

use Laravel\Prompts\Output\BufferedConsoleOutput;
use Laravel\Prompts\Terminal;
use PHPUnit\Framework\Assert;
use RuntimeException;

trait FakesInputOutput
{
    /**
     * Fake the terminal and queue key presses to be simulated.
     *
     * @param  array<string>  $keys
     */
    public static function fake(array $keys = []): void
    {}

    /**
     * Implementation of the looping mechanism for simulating key presses.
     *
     * By ignoring the `$callable` parameter which contains the default logic
     * for simulating fake key presses, we can use a custom implementation
     * to emit key presses instead, allowing us to use different inputs.
     *
     * @param  array<string>  $keys
     * @param  callable(string $key): void  $callable
     */
    public static function fakeKeyPresses(array $keys, callable $callable): void
    {}

    /**
     * Assert that the output contains the given string.
     */
    public static function assertOutputContains(string $string): void
    {}

    /**
     * Assert that the output doesn't contain the given string.
     */
    public static function assertOutputDoesntContain(string $string): void
    {}

    /**
     * Assert that the stripped output contains the given string.
     */
    public static function assertStrippedOutputContains(string $string): void
    {}

    /**
     * Assert that the stripped output doesn't contain the given string.
     */
    public static function assertStrippedOutputDoesntContain(string $string): void
    {}

    /**
     * Get the buffered console output.
     */
    public static function content(): string
    {}

    /**
     * Get the buffered console output, stripped of escape sequences.
     */
    public static function strippedContent(): string
    {}
}
