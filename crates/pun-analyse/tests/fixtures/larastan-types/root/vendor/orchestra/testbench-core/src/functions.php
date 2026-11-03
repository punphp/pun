<?php

namespace Orchestra\Testbench;

use Closure;
use Illuminate\Contracts\Console\Kernel as ConsoleKernel;
use Illuminate\Contracts\Foundation\Application as ApplicationContract;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Foundation\Application;
use Illuminate\Routing\Router;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\ParallelTesting;
use Illuminate\Support\ProcessUtils;
use Illuminate\Support\Str;
use Illuminate\Testing\PendingCommand;
use InvalidArgumentException;
use Orchestra\Sidekick;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;
use PHPUnit\Runner\ShutdownHandler;

/**
 * Create Laravel application instance.
 *
 * @api
 *
 * @param  string|null  $basePath
 * @param  (callable(\Illuminate\Foundation\Application):(void))|null  $resolvingCallback
 * @param  array{extra?: array{providers?: array, dont-discover?: array, env?: array}, load_environment_variables?: bool, enabled_package_discoveries?: bool}  $options
 * @param  \Orchestra\Testbench\Foundation\Config|null  $config
 * @return \Orchestra\Testbench\Foundation\Application
 */
function container(
    ?string $basePath = null,
    ?callable $resolvingCallback = null,
    array $options = [],
    ?Foundation\Config $config = null
): Foundation\Application {}

/**
 * Run artisan command.
 *
 * @api
 *
 * @param  \Orchestra\Testbench\Contracts\TestCase|\Illuminate\Contracts\Foundation\Application  $context
 * @param  string  $command
 * @param  array<string, mixed>  $parameters
 * @return int
 */
function artisan(Contracts\TestCase|ApplicationContract $context, string $command, array $parameters = []): int
{}

/**
 * Emit an exit event within a test.
 *
 * @param  \PHPUnit\Framework\TestCase|object|null  $testCase
 * @param  string|int  $status
 * @return never
 */
function bail(?object $testCase, string|int $status = 0): never
{}

/**
 * Emit an exit event within a test.
 *
 * @param  \PHPUnit\Framework\TestCase|object|null  $testCase
 * @param  string|int  $status
 * @return never
 */
function terminate(?object $testCase, string|int $status = 0): never
{}

/**
 * Run remote action using Testbench CLI.
 *
 * @api
 *
 * @param  (\Closure():(mixed))|array<int, string>|string  $command
 * @param  array<string, mixed>|string  $env
 * @param  bool|null  $tty
 * @return \Orchestra\Testbench\Foundation\Process\ProcessDecorator
 */
function remote(Closure|array|string $command, array|string $env = [], ?bool $tty = null): Foundation\Process\ProcessDecorator
{}

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
 * @return void
 */
function after_resolving(ApplicationContract $app, string $name, ?Closure $callback = null): void
{}

/**
 * Load migration paths.
 *
 * @api
 *
 * @param  \Illuminate\Contracts\Foundation\Application  $app
 * @param  array<int, string>|string  $paths
 * @return void
 */
function load_migration_paths(ApplicationContract $app, array|string $paths): void
{}

/**
 * Get defined environment variables.
 *
 * @api
 *
 * @return array<string, mixed>
 */
function defined_environment_variables(): array
{}

/**
 * Get default environment variables.
 *
 * @api
 *
 * @param  iterable<string, mixed>  $variables
 * @return array<int, string>
 */
function parse_environment_variables($variables): array
{}

/**
 * Refresh router lookups.
 *
 * @api
 *
 * @param  \Illuminate\Routing\Router  $router
 * @return void
 */
function refresh_router_lookups(Router $router): void
{}

/**
 * Transform realpath to alias path.
 *
 * @api
 *
 * @param  string  $path
 * @param  string|null  $workingPath
 * @return string
 */
function transform_realpath_to_relative(string $path, ?string $workingPath = null, string $prefix = ''): string
{}

/**
 * Get the default skeleton path.
 *
 * @api
 *
 * @no-named-arguments
 *
 * @param  array<int, string|null>|string  ...$path
 * @return ($path is '' ? string : string|false)
 */
function default_skeleton_path(array|string $path = ''): string|false
{}

/**
 * Determine if application is bootstrapped using Testbench's default skeleton.
 *
 * @param  string|null  $basePath
 * @return bool
 */
function uses_default_skeleton(?string $basePath = null): bool
{}

/**
 * Get the migration path by type.
 *
 * @api
 *
 * @param  string|null  $type
 * @return string
 *
 * @throws \InvalidArgumentException
 */
function default_migration_path(?string $type = null): string
{}

/**
 * Get the path to the package folder.
 *
 * @api
 *
 * @no-named-arguments
 *
 * @param  array<int, string|null>|string  ...$path
 * @return string
 */
function package_path(array|string $path = ''): string
{}

/**
 * Get the workbench configuration.
 *
 * @api
 *
 * @return array<string, mixed>
 */
function workbench(): array
{}

/**
 * Get the path to the workbench folder.
 *
 * @api
 *
 * @no-named-arguments
 *
 * @param  array<int, string|null>|string  ...$path
 * @return string
 */
function workbench_path(array|string $path = ''): string
{}

/**
 * Determine if vendor symlink exists on the laravel application.
 *
 * @api
 *
 * @param  \Illuminate\Contracts\Foundation\Application  $app
 * @param  string|null  $workingPath
 * @return bool
 */
function laravel_vendor_exists(ApplicationContract $app, ?string $workingPath = null): bool
{}

/**
 * Laravel version compare.
 *
 * @api
 *
 * @template TOperator of string|null
 *
 * @param  string  $version
 * @param  string|null  $operator
 *
 * @phpstan-param  TOperator  $operator
 *
 * @return int|bool
 *
 * @phpstan-return (TOperator is null ? int : bool)
 *
 * @codeCoverageIgnore
 */
function laravel_version_compare(string $version, ?string $operator = null): int|bool
{}

/**
 * Package version compare.
 *
 * @api
 *
 * @template TOperator of string|null
 *
 * @param  string  $package
 * @param  string  $version
 * @param  string|null  $operator
 *
 * @phpstan-param  TOperator  $operator
 *
 * @return int|bool
 *
 * @phpstan-return (TOperator is null ? int : bool)
 *
 * @throws \OutOfBoundsException
 * @throws \RuntimeException
 *
 * @codeCoverageIgnore
 */
function package_version_compare(string $package, string $version, ?string $operator = null)
{}

/**
 * PHP version compare.
 *
 * @api
 *
 * @template TOperator of string|null
 *
 * @param  string  $version
 * @param  string|null  $operator
 *
 * @phpstan-param  TOperator  $operator
 *
 * @return int|bool
 *
 * @phpstan-return (TOperator is null ? int : bool)
 */
function php_version_compare(string $version, ?string $operator = null)
{}

/**
 * PHPUnit version compare.
 *
 * @api
 *
 * @template TOperator of string|null
 *
 * @param  string  $version
 * @param  string|null  $operator
 *
 * @phpstan-param  TOperator  $operator
 *
 * @return int|bool
 *
 * @phpstan-return (TOperator is null ? int : bool)
 *
 * @throws \OutOfBoundsException
 * @throws \RuntimeException
 *
 * @codeCoverageIgnore
 */
function phpunit_version_compare(string $version, ?string $operator = null): int|bool
{}

/**
 * Determine the PHP Binary.
 *
 * @api
 *
 * @param  bool  $escape
 * @return string
 */
function php_binary(bool $escape = false): string
{}

/**
 * Join the given paths together.
 *
 * @param  string|null  $basePath
 * @param  string  ...$paths
 * @return string
 *
 * @codeCoverageIgnore
 */
function join_paths(?string $basePath, string ...$paths): string
{}

/**
 * Ensure the provided `$app` return an instance of Laravel application or throw an exception.
 *
 * @internal
 *
 * @param  \Illuminate\Foundation\Application|null  $app
 * @param  string|null  $caller
 * @return \Illuminate\Foundation\Application
 *
 * @throws \Orchestra\Testbench\Exceptions\ApplicationNotAvailableException
 */
function laravel_or_fail($app, ?string $caller = null): Application
{}

/**
 * Determine if current test is handled via parallel testing.
 *
 * @api
 *
 * @return bool
 */
function in_parallel_testing(): bool
{}
