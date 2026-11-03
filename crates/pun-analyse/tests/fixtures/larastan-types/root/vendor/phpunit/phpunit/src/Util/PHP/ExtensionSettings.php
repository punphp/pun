<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util\PHP;

use const PHP_BINARY;
use const PHP_SAPI;
use function array_is_list;
use function array_keys;
use function array_merge;
use function fclose;
use function get_loaded_extensions;
use function in_array;
use function is_array;
use function is_resource;
use function is_string;
use function json_decode;
use function proc_close;
use function proc_open;
use function stream_get_contents;
use function strtolower;
use Closure;
use ReflectionExtension;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ExtensionSettings
{
    /**
     * @var ?list<non-empty-string>
     */
    private static ?array $settings = null;

    /**
     * Returns the "extension" and "zend_extension" settings that a child
     * process needs to load the extensions that are loaded in the current
     * process but that it does not load from the INI files, for instance
     * because they were loaded using "php -d extension=...".
     *
     * PHP does not expose these settings, which is why they are inferred by
     * comparing the extensions loaded in the current process with those loaded
     * in a child process. A setting is only returned when a child process that
     * is started with it actually loads the extension: the warning that PHP
     * emits when an extension cannot be loaded would otherwise make every test
     * that is run in a child process error.
     *
     * Extensions loaded using dl() are not considered: the child process runs
     * the code that loads them, too.
     *
     * The settings are determined when this method is first called and then
     * reused.
     *
     * @param Closure(): list<string> $parameters returns the -d parameters (other than the ones for the settings returned by this method) that child processes are started with
     *
     * @return list<non-empty-string>
     */
    public static function forChildProcess(Closure $parameters): array
    {}

    /**
     * @param Closure(): list<string> $parameters
     *
     * @return list<non-empty-string>
     */
    private static function settings(Closure $parameters): array
    {}

    /**
     * @param list<string> $parameters
     *
     * @return ?list<string>
     */
    private static function loadedExtensionsInChildProcess(array $parameters): ?array
    {}

    /**
     * Maps the name of an extension to the name that the "extension" and
     * "zend_extension" settings resolve to a file in the extension directory.
     */
    private static function fileName(string $name): string
    {}
}
