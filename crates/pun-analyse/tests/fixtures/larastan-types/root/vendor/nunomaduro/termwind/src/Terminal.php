<?php

declare(strict_types=1);

namespace Termwind;

use Symfony\Component\Console\Terminal as ConsoleTerminal;

/**
 * @internal
 */
final class Terminal
{
    /**
     * An instance of Symfony's console terminal.
     */
    private ConsoleTerminal $terminal;

    /**
     * Creates a new terminal instance.
     */
    public function __construct(?ConsoleTerminal $terminal = null)
    {}

    /**
     * Gets the terminal width.
     */
    public function width(): int
    {}

    /**
     * Gets the terminal height.
     */
    public function height(): int
    {}

    /**
     * Clears the terminal screen.
     */
    public function clear(): void
    {}
}
