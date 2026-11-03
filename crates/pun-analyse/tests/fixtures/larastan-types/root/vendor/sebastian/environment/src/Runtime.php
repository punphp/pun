<?php declare(strict_types=1);
/*
 * This file is part of sebastian/environment.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Environment;

use const INI_SCANNER_NORMAL;
use const PHP_BINARY;
use const PHP_SAPI;
use const PHP_VERSION;
use function array_key_exists;
use function array_map;
use function array_merge;
use function assert;
use function escapeshellarg;
use function explode;
use function extension_loaded;
use function in_array;
use function ini_get;
use function ini_get_all;
use function is_array;
use function is_int;
use function is_string;
use function json_decode;
use function parse_ini_file;
use function php_ini_loaded_file;
use function php_ini_scanned_files;
use function phpversion;
use function proc_close;
use function proc_open;
use function reset;
use function sprintf;
use function stream_get_contents;
use function strrpos;
use function version_compare;
use function xdebug_info;

final class Runtime
{
    /**
     * Settings that the CLI SAPI overrides at startup, before any
     * php.ini file is loaded (see HARDCODED_INI and
     * sapi_cli_ini_defaults() in sapi/cli/php_cli.c). Their
     * `builtin_default_value` reported by ini_get_all() is not
     * the value a `php` child process starts with.
     */
    private const array CLI_SAPI_DEFAULTS = [
        'display_errors'     => '1',
        'html_errors'        => '0',
        'implicit_flush'     => '1',
        'max_execution_time' => '0',
        'max_input_time'     => '-1',
        'output_buffering'   => '0',
    ];

    /**
     * @var ?array<string, string>
     */
    private static ?array $compiledDefaults = null;

    /**
     * Returns true when Xdebug or PCOV is available or
     * the runtime used is PHPDBG.
     */
    public function canCollectCodeCoverage(): bool
    {}

    /**
     * Returns true when Zend OPcache is loaded, enabled,
     * and is configured to discard comments.
     */
    public function discardsComments(): bool
    {}

    /**
     * Returns true when Zend OPcache is loaded, enabled,
     * and is configured to perform just-in-time compilation.
     */
    public function performsJustInTimeCompilation(): bool
    {}

    /**
     * Returns the raw path to the binary of the current runtime.
     *
     * @deprecated
     */
    public function getRawBinary(): string
    {}

    /**
     * Returns the escaped path to the binary of the current runtime.
     *
     * @deprecated
     */
    public function getBinary(): string
    {}

    public function getNameWithVersion(): string
    {}

    public function getNameWithVersionAndCodeCoverageDriver(): string
    {}

    public function getName(): string
    {}

    public function getVendorUrl(): string
    {}

    public function getVersion(): string
    {}

    /**
     * Returns true when the runtime used is PHP and Xdebug is loaded.
     */
    public function hasXdebug(): bool
    {}

    /**
     * Returns true when the runtime used is PHP without the PHPDBG SAPI.
     */
    public function isPHP(): bool
    {}

    /**
     * Returns true when the runtime used is PHP with the PHPDBG SAPI.
     */
    public function isPHPDBG(): bool
    {}

    /**
     * Returns true when the runtime used is PHP with the PHPDBG SAPI
     * and the phpdbg_*_oplog() functions are available (PHP >= 7.0).
     */
    public function hasPHPDBGCodeCoverage(): bool
    {}

    /**
     * Returns true when the runtime used is PHP with PCOV loaded and enabled.
     */
    public function hasPCOV(): bool
    {}

    /**
     * Parses the loaded php.ini file (if any) as well as all
     * additional php.ini files from the additional ini dir into a
     * single merged map of settings, and also obtains the compiled-in
     * defaults (read in-process from ini_get_all() where available,
     * otherwise by spawning a `php -n` child once per process).
     * Then checks for each setting passed via the `$values` parameter
     * whether the runtime value (`ini_get()`) differs from what the
     * ini files specified or, when a setting is not configured in any
     * ini file, from the compiled-in default. Returns an array of
     * `key=value` strings for the changed settings.
     *
     * A setting whose runtime value is the empty string but that is
     * absent from both the ini files and the compiled-in defaults is
     * left alone: there is no evidence that it was overridden, so it
     * is not forwarded. This avoids spurious empty overrides for
     * settings of extensions that are not visible to the compiled-in
     * defaults probe (e.g. extensions loaded only via php.ini), which
     * would otherwise break child processes (see
     * https://github.com/sebastianbergmann/environment/issues/99).
     *
     * @param list<string> $values
     *
     * @return array<string, string>
     */
    public function getCurrentSettings(array $values): array
    {}

    /**
     * Returns INI settings that cannot be changed via ini_set()
     * (PHP_INI_SYSTEM and PHP_INI_PERDIR) and whose current value
     * differs from the value configured in INI files.
     *
     * These settings can only have been changed via CLI -d flags
     * and must be forwarded as -d flags to child processes because
     * ini_set() cannot change them at runtime.
     *
     * @return array<string, string>
     */
    public function getSettingsNotChangeableAtRuntime(): array
    {}

    public function isOpcacheActive(): bool
    {}

    /**
     * @return array<array-key, string>
     */
    private function parseLoadedIniFiles(): array
    {}

    /**
     * Returns the compiled-in default values of every ini setting,
     * caching the result for the lifetime of the PHP process.
     *
     * On PHP versions where ini_get_all() exposes the
     * `builtin_default_value` of each setting (see
     * https://github.com/php/php-src/issues/22133) the defaults are
     * read in-process. On older versions they are
     * obtained by spawning a `php -n` child process once, which is
     * avoided where possible because forking can trigger unwanted
     * side effects from extension fork handlers.
     *
     * When running on the CLI SAPI, the in-process defaults are
     * amended with the settings the CLI SAPI overrides at startup so
     * that they match what a `php -n` child process would report.
     *
     * @return array<string, string>
     */
    private static function compiledDefaults(): array
    {}

    /**
     * Returns the compiled-in default values of every ini setting by
     * spawning a `php -n` child process once and caching the result
     * for the lifetime of the PHP process. When the child cannot be
     * launched or its output is unusable, an empty array is returned
     * and cached so the failure is not retried.
     *
     * @return array<string, string>
     */
    private static function compiledDefaultsViaChildProcess(): array
    {}
}
