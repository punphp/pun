<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util;

use const PHP_MAJOR_VERSION;
use const PHP_MINOR_VERSION;
use function array_reverse;
use function array_shift;
use function assert;
use function defined;
use function get_defined_constants;
use function get_included_files;
use function in_array;
use function ini_get_all;
use function is_array;
use function is_file;
use function is_scalar;
use function is_string;
use function preg_match;
use function serialize;
use function sprintf;
use function str_ends_with;
use function str_starts_with;
use function strtr;
use function var_export;
use Closure;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class GlobalState
{
    /**
     * @var non-empty-list<non-empty-string>
     */
    private const array SUPER_GLOBAL_ARRAYS = [
        '_ENV',
        '_POST',
        '_GET',
        '_COOKIE',
        '_SERVER',
        '_FILES',
        '_REQUEST',
    ];

    /**
     * Keys in $_SERVER that are populated by the SAPI in the child process
     * and must therefore not be preserved from the parent process.
     *
     * @var non-empty-list<non-empty-string>
     */
    private const array SAPI_SERVER_KEYS = [
        'PHP_SELF',
        'SCRIPT_NAME',
        'SCRIPT_FILENAME',
        'PATH_TRANSLATED',
        'DOCUMENT_ROOT',
        'REQUEST_TIME',
        'REQUEST_TIME_FLOAT',
        'argv',
        'argc',
    ];

    /**
     * @var non-empty-array<non-empty-string, non-empty-array<non-empty-string, true>>
     */
    private const array DEPRECATED_INI_SETTINGS = [
        '8.4' => [
            'auto_detect_line_endings'     => true,
            'filter.default'               => true,
            'iconv.input_encoding'         => true,
            'iconv.output_encoding'        => true,
            'iconv.internal_encoding'      => true,
            'mbstring.http_input'          => true,
            'mbstring.http_output'         => true,
            'mbstring.internal_encoding'   => true,
            'oci8.old_oci_close_semantics' => true,
        ],

        '8.5' => [
            'auto_detect_line_endings'     => true,
            'filter.default'               => true,
            'iconv.input_encoding'         => true,
            'iconv.output_encoding'        => true,
            'iconv.internal_encoding'      => true,
            'mbstring.http_input'          => true,
            'mbstring.http_output'         => true,
            'mbstring.internal_encoding'   => true,
            'oci8.old_oci_close_semantics' => true,
        ],

        '8.6' => [
            'auto_detect_line_endings'     => true,
            'filter.default'               => true,
            'iconv.input_encoding'         => true,
            'iconv.output_encoding'        => true,
            'iconv.internal_encoding'      => true,
            'mbstring.http_input'          => true,
            'mbstring.http_output'         => true,
            'mbstring.internal_encoding'   => true,
            'oci8.old_oci_close_semantics' => true,
        ],
    ];

    /**
     * @throws Exception
     */
    public static function getIncludedFilesAsString(): string
    {}

    /**
     * @param list<string> $files
     *
     * @throws Exception
     */
    public static function processIncludedFilesAsString(array $files): string
    {}

    public static function getIniSettingsAsString(): string
    {}

    public static function getConstantsAsString(): string
    {}

    public static function exportGlobals(): GlobalStateResult
    {}

    private static function exportVariable(mixed $variable): string
    {}

    /**
     * @param array<mixed> $array
     */
    private static function arrayOnlyContainsScalars(array $array): bool
    {}

    private static function isIniSettingDeprecated(string $iniSetting): bool
    {}
}
