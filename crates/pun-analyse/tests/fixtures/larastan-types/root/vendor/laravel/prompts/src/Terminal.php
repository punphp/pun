<?php

namespace Laravel\Prompts;

use ReflectionClass;
use RuntimeException;
use Symfony\Component\Console\Terminal as SymfonyTerminal;

class Terminal
{
    /**
     * The initial TTY mode.
     */
    protected ?string $initialTtyMode;

    /**
     * Whether the terminal supports true color.
     */
    protected static ?bool $trueColorSupport = null;

    /**
     * The terminal's foreground color as an RGB array.
     *
     * @var array{int, int, int}|null
     */
    protected static ?array $foregroundColor = null;

    /**
     * The terminal's background color as an RGB array.
     *
     * @var array{int, int, int}|null
     */
    protected static ?array $backgroundColor = null;

    /**
     * The Symfony Terminal instance.
     */
    protected SymfonyTerminal $terminal;

    /**
     * Create a new Terminal instance.
     */
    public function __construct()
    {}

    /**
     * Read a line from the terminal.
     */
    public function read(): string
    {}

    /**
     * Set the TTY mode.
     */
    public function setTty(string $mode): void
    {}

    /**
     * Restore the initial TTY mode.
     */
    public function restoreTty(): void
    {}

    /**
     * Get the number of columns in the terminal.
     */
    public function cols(): int
    {}

    /**
     * Get the number of lines in the terminal.
     */
    public function lines(): int
    {}

    /**
     * (Re)initialize the terminal dimensions.
     */
    public function initDimensions(): void
    {}

    /**
     * Exit the interactive session.
     */
    public function exit(): void
    {}

    /**
     * Execute the given command and return the output.
     */
    protected function exec(string $command): string
    {}

    /**
     * Determine if the terminal supports true color (24-bit).
     */
    public function supportsTrueColor(): bool
    {}

    /**
     * Get the terminal's foreground color as an RGB array.
     *
     * @return array{int, int, int}
     */
    public function foregroundColor(): array
    {}

    /**
     * Get the terminal's background color as an RGB array.
     *
     * @return array{int, int, int}
     */
    public function backgroundColor(): array
    {}

    /**
     * Query the terminal for foreground and background colors in a single shot.
     */
    protected function queryColors(): void
    {}
}
