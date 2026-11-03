<?php

namespace Illuminate\Foundation;

class DevCommand
{
    /**
     * The priority level for default commands that are registered by the framework.
     */
    const PRIORITY_DEFAULT = 0;

    /**
     * The priority level for commands that are registered by packages in the vendor directory.
     */
    const PRIORITY_VENDOR = 1;

    /**
     * The priority level for commands that are registered by the user in their application.
     */
    const PRIORITY_USERLAND = 2;

    /**
     * Color of the command when output to the console.
     *
     * @var string|null
     */
    protected ?string $color = null;

    /**
     * Create a new DevCommand instance.
     *
     * @param  string  $command
     * @param  array{'file': string, 'line': int, 'class'?: string, 'function'?: string}  $source
     * @param  string|null  $name
     * @param  self::PRIORITY_DEFAULT|self::PRIORITY_VENDOR|self::PRIORITY_USERLAND  $priority
     * @return void
     */
    public function __construct(
        protected string $command,
        protected array $source,
        protected ?string $name = null,
        protected int $priority = self::PRIORITY_USERLAND,
    ) {}

    /**
     * Derive the name from a command string.
     *
     * @param  string  $command
     * @return string
     */
    public static function nameFromCommand(string $command): string
    {}

    /**
     * Get the command name.
     *
     * @return string
     */
    public function name(): string
    {}

    /**
     * Get the command priority.
     *
     * @return int
     */
    public function priority(): int
    {}

    /**
     * Set the command color.
     *
     * @param  string  $color
     * @return self
     */
    public function color(string $color): self
    {}

    /**
     * Set the command color to blue.
     *
     * @return self
     */
    public function blue(): self
    {}

    /**
     * Set the command color to purple.
     *
     * @return self
     */
    public function purple(): self
    {}

    /**
     * Set the command color to pink.
     *
     * @return self
     */
    public function pink(): self
    {}

    /**
     * Set the command color to orange.
     *
     * @return self
     */
    public function orange(): self
    {}

    /**
     * Set the command color to green.
     *
     * @return self
     */
    public function green(): self
    {}

    /**
     * Set the command color to yellow.
     *
     * @return self
     */
    public function yellow(): self
    {}

    /**
     * Get the command as an array.
     *
     * @return array{command: string, name: string, color: string|null, source: array{'file': string, 'line': int, 'class'?: string, 'function'?: string}, priority: int}
     */
    public function toArray(): array
    {}
}
