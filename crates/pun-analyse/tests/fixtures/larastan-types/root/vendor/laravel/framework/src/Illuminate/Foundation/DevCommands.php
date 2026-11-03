<?php

namespace Illuminate\Foundation;

use Illuminate\Support\Arr;
use Illuminate\Support\Facades\File;
use Illuminate\Support\NodePackageManager;
use Laravel\Pail\PailServiceProvider;
use ReflectionClass;

/**
 * @phpstan-type DevCommandArray array{'name': string, 'command': string, 'source': array{'file': string, 'line': int, 'class'?: string, 'function'?: string}, 'color': string}
 */
class DevCommands
{
    /**
     * The resolved NodePackageManager instance.
     *
     * @var NodePackageManager|null
     */
    protected static ?NodePackageManager $packageManager = null;

    /**
     * Counter to keep track of how many colors have been assigned.
     *
     * Used to ensure colors are reused only after all have been used at least once.
     *
     * @var int
     */
    protected static $colorCount = 0;

    /**
     * The registered development commands.
     *
     * @var array
     */
    protected static $commands = [];

    /**
     * The names of commands that should be included when running the "dev" command.
     *
     * @var array<int, string>
     */
    protected static $only = [];

    /**
     * The names of commands that should be excluded when running the "dev" command.
     *
     * @var array<int, string>
     */
    protected static $except = [];

    /**
     * The order in which named commands should run when running the "dev" command.
     *
     * @var array<int, string>
     */
    protected static $order = [];

    /**
     * The mode in which the "dev" command should run.
     *
     * @var DevCommandMode
     */
    protected static DevCommandMode $mode = DevCommandMode::TABS;

    /**
     * Whether to include timestamps in the output of the "dev" command.
     *
     * @var bool
     */
    protected static $withTimestamps = false;

    /**
     * Whether to automatically restart a "dev" command when it fails.
     *
     * @var bool
     */
    protected static $autoRestart = true;

    /**
     * Whether to exclude vendor commands.
     *
     * @var bool
     */
    protected static $withoutVendorCommands = false;

    /**
     * Whether to exclude the framework's default commands.
     *
     * @var bool
     */
    protected static $withoutDefaultCommands = false;

    /**
     * How many lines of output to buffer for each command when running in tabbed mode.
     *
     * @var int|null
     */
    protected static ?int $bufferSize = null;

    /**
     * How many lines of output to buffer total when running in stream mode.
     *
     * @var int|null
     */
    protected static ?int $streamBufferSize = null;

    /**
     * Register the default development commands.
     *
     * @return void
     */
    public static function registerDefaults()
    {}

    /**
     * Register a development command.
     *
     * @param  string  $command
     * @param  string|null  $name
     * @return DevCommand
     */
    public static function register(string $command, ?string $name = null): DevCommand
    {}

    /**
     * Registers an Artisan command, automatically prefixing it with "php artisan".
     *
     * @param  string  $command
     * @param  string|null  $name
     * @return DevCommand
     */
    public static function artisan(string $command, ?string $name = null): DevCommand
    {}

    /**
     * Registers a Node command, automatically prefixing it with the detected package manager's run command.
     *
     * @param  string  $command
     * @param  string|null  $name
     * @return DevCommand
     */
    public static function node(string $command, ?string $name = null): DevCommand
    {}

    /**
     * Registers a Node command, automatically prefixing it with the detected package manager's exec command.
     *
     * @param  string  $command
     * @param  string|null  $name
     * @return DevCommand
     */
    public static function nodeExec(string $command, ?string $name = null): DevCommand
    {}

    /**
     * Get the registered development commands.
     *
     * @return DevCommandArray[]
     */
    public static function commands(): array
    {}

    /**
     * Set the mode to inline, where all commands are run in the same terminal window.
     *
     * No-op on Windows.
     *
     * @return void
     */
    public static function inline(): void
    {}

    /**
     * Set the mode to stream, where all commands are run in the same terminal window, but their output is interactive within a TUI.
     *
     * No-op on Windows.
     *
     * @return void
     */
    public static function stream(): void
    {}

    /**
     * Set the mode to tabs, where each command is run in its own terminal tab.
     *
     * No-op on Windows.
     *
     * @return void
     */
    public static function tabs(): void
    {}

    /**
     * Get the mode in which the "dev" command should run.
     *
     * @return DevCommandMode
     */
    public static function mode(): DevCommandMode
    {}

    /**
     * Enable timestamps in the output of the "dev" command.
     *
     * @return void
     */
    public static function withTimestamps(): void
    {}

    /**
     * Determine if timestamps should be included in the output of the "dev" command.
     *
     * @return bool
     */
    public static function shouldIncludeTimestamps(): bool
    {}

    /**
     * Disable automatic restart of a "dev" command when it fails.
     *
     * @return void
     */
    public static function disableAutoRestart(): void
    {}

    /**
     * Determine if a "dev" command should automatically restart when it fails.
     *
     * @return bool
     */
    public static function shouldAutoRestart(): bool
    {}

    /**
     * Set the number of lines of output to buffer for each command when running in tabbed mode.
     *
     * No-op on Windows.
     *
     * @param  int  $lines
     * @return void
     */
    public static function bufferSize(int $lines): void
    {}

    /**
     * Get the number of lines of output to buffer for each command when running in tabbed mode.
     *
     * @return int|null
     */
    public static function getBufferSize(): ?int
    {}

    /**
     * Set the number of lines of output to buffer total when running in stream mode.
     *
     * No-op on Windows.
     *
     * @param  int  $lines
     * @return void
     */
    public static function streamBufferSize(int $lines): void
    {}

    /**
     * Get the number of lines of output to buffer total when running in stream mode.
     *
     * @return int|null
     */
    public static function getStreamBufferSize(): ?int
    {}

    /**
     * Sort the given commands.
     *
     * @param  array  $commands
     * @return array
     */
    protected static function applyOrder(array $commands): array
    {}

    /**
     * Fill in any empty colors in the given commands array, ensuring each command has a color assigned.
     *
     * @param  array  $commands
     * @return array
     */
    protected static function fillInEmptyColors(array $commands): array
    {}

    /**
     * Get a color for a command, ensuring that colors are reused only after all available colors have been used at least once.
     *
     * @param  array  $commands
     * @return string
     */
    protected static function getColor(array $commands): string
    {}

    /**
     * Resolve the first external caller frame from a debug backtrace.
     *
     * @param  array<int, array{'file': string, 'line': int, 'class'?: string, 'function'?: string}>  $trace
     * @return array{'file': string, 'line': int, 'class'?: string, 'function'?: string}
     */
    protected static function resolveSource(array $trace): array
    {}

    /**
     * Determine the registration priority from a debug backtrace.
     *
     * @param  array<int, array{'file': string, 'line': int, 'class'?: string, 'function'?: string}>  $trace
     * @return int
     */
    protected static function resolvePriority(array $trace): int
    {}

    /**
     * Set the commands that should be included when running the "dev" command.
     *
     * @param  string  ...$names
     * @return void
     */
    public static function only(...$names): void
    {}

    /**
     * Set the commands that should be excluded when running the "dev" command.
     *
     * @param  string  ...$names
     * @return void
     */
    public static function except(...$names): void
    {}

    /**
     * Set the order in which named commands should run when running the "dev" command.
     *
     * @param  array<int, string>  $names
     * @return void
     */
    public static function order(array $names): void
    {}

    /**
     * Exclude any commands from the vendor directory.
     *
     * @return void
     */
    public static function withoutVendorCommands(): void
    {}

    /**
     * Exclude the framework's default commands.
     *
     * @return void
     */
    public static function withoutDefaultCommands(): void
    {}

    /**
     * Resolve and return the NodePackageManager instance.
     *
     * @return NodePackageManager
     */
    protected static function getPackageManager(): NodePackageManager
    {}
}
