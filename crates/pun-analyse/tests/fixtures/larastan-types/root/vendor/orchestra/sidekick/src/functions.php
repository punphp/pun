<?php

namespace Orchestra\Sidekick;

use BackedEnum;
use Closure;
use Composer\InstalledVersions;
use Composer\Semver\Semver;
use Composer\Semver\VersionParser;
use Illuminate\Contracts\Foundation\Application as ApplicationContract;
use Illuminate\Foundation\Application;
use Illuminate\Support\Arr;
use OutOfBoundsException;
use PHPUnit\Runner\Version;
use RuntimeException;
use UnitEnum;

if (! \function_exists('Orchestra\Sidekick\enum_name')) {
    /**
     * Get the proper name from enum.
     *
     * @api
     *
     * @throws \RuntimeException
     */
    function enum_name(BackedEnum|UnitEnum $enum): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\enum_value')) {
    /**
     * Get the proper name from enum.
     *
     * @api
     *
     * @template TValue
     * @template TDefault
     *
     * @param  TValue  $value
     * @param  TDefault|callable(TValue): TDefault  $default
     * @return ($value is empty ? TDefault : mixed)
     *
     * @throws \RuntimeException
     */
    function enum_value(mixed $value, mixed $default = null): mixed
    {}
}

if (! \function_exists('Orchestra\Sidekick\after_resolving')) {
    /**
     * Register after resolving callback.
     *
     * @api
     *
     * @template TLaravel of \Illuminate\Contracts\Foundation\Application
     *
     * @param  TLaravel  $app
     * @param  class-string|string  $name
     * @param  (\Closure(object, TLaravel):(mixed))|null  $callback
     */
    function after_resolving(ApplicationContract $app, string $name, ?Closure $callback = null): void
    {}
}

if (! \function_exists('Orchestra\Sidekick\join_paths')) {

    /**
     * Join the given paths together.
     *
     * @api
     *
     * @see \Orchestra\Sidekick\Filesystem\join_paths
     */
    function join_paths(?string $basePath, string ...$paths): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\once')) {
    /**
     * Run callback only once.
     *
     * @api
     *
     * @param  mixed  $callback
     * @return \Closure():mixed
     */
    function once($callback): Closure
    {}
}

if (! \function_exists('Orchestra\Sidekick\is_safe_callable')) {
    /**
     * Determine if the value is a callable and not a string matching an available function name.
     *
     * @api
     */
    function is_safe_callable(mixed $value): bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\is_symlink')) {
    /**
     * Determine if the path is a symlink for both Unix and Windows environments.
     *
     * @api
     *
     * @see \Orchestra\Sidekick\Filesystem\is_symlink
     */
    function is_symlink(string $path): bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\is_testbench_cli')) {
    /**
     * Determine if command executed via Testbench CLI.
     *
     * @api
     */
    function is_testbench_cli(?bool $dusk = null): bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\transform_relative_path')) {
    /**
     * Transform relative path.
     *
     * @api
     */
    function transform_relative_path(string $path, string $workingPath): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\package_path')) {
    /**
     * Get the package path.
     *
     * @api
     *
     * @no-named-arguments
     *
     * @param  array<int, string|null>|string  ...$path
     */
    function package_path(array|string $path = ''): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\working_path')) {
    /**
     * Get the working path.
     *
     * @api
     *
     * @no-named-arguments
     *
     * @param  array<int, string|null>|string  ...$path
     */
    function working_path(array|string $path = ''): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\laravel_normalize_version')) {
    /**
     * Laravel normalize version.
     *
     * @api
     *
     * @throws \OutOfBoundsException
     */
    function laravel_normalize_version(): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\php_normalize_version')) {
    /**
     * PHP normalize version.
     *
     * @api
     */
    function php_normalize_version(): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\phpunit_normalize_version')) {
    /**
     * PHPUnit normalize version.
     *
     * @api
     *
     * @throws \OutOfBoundsException
     */
    function phpunit_normalize_version(): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\laravel_version_compare')) {
    /**
     * Laravel version compare.
     *
     * @api
     *
     * @template TOperator of string|null
     *
     * @param  TOperator  $operator
     * @return (TOperator is null ? int : bool)
     *
     * @throws \RuntimeException
     *
     * @codeCoverageIgnore
     */
    function laravel_version_compare(string $version, ?string $operator = null): int|bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\php_version_compare')) {
    /**
     * PHP version compare.
     *
     * @api
     *
     * @template TOperator of string|null
     *
     * @param  TOperator  $operator
     * @return (TOperator is null ? int : bool)
     *
     * @codeCoverageIgnore
     */
    function php_version_compare(string $version, ?string $operator = null): int|bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\package_version_compare')) {
    /**
     * Package version compare.
     *
     * @api
     *
     * @template TOperator of string|null
     *
     * @phpstan-param  TOperator  $operator
     *
     * @phpstan-return (TOperator is null ? int : bool)
     *
     * @throws \OutOfBoundsException
     * @throws \RuntimeException
     *
     * @codeCoverageIgnore
     */
    function package_version_compare(string $package, string $version, ?string $operator = null): int|bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\phpunit_version_compare')) {
    /**
     * PHPUnit version compare.
     *
     * @api
     *
     * @template TOperator of string|null
     *
     * @param  TOperator  $operator
     * @return (TOperator is null ? int : bool)
     *
     * @throws \OutOfBoundsException
     * @throws \RuntimeException
     *
     * @codeCoverageIgnore
     */
    function phpunit_version_compare(string $version, ?string $operator = null): int|bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\php_binary')) {
    /**
     * Determine the PHP Binary.
     *
     * @api
     *
     * @codeCoverageIgnore
     */
    function php_binary(): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\windows_os')) {
    /**
     * Determine whether the current environment is Windows-based.
     *
     * @api
     *
     * @codeCoverageIgnore
     */
    function windows_os(): bool
    {}
}
