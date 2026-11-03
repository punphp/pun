<?php declare(strict_types=1);

/*
 * This file is part of the Monolog package.
 *
 * (c) Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Monolog;

use Monolog\Formatter\FormatterInterface;
use Monolog\Formatter\WrappingFormatterInterface;

final class Utils
{
    const DEFAULT_JSON_FLAGS = JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_PRESERVE_ZERO_FRACTION | JSON_INVALID_UTF8_SUBSTITUTE | JSON_PARTIAL_OUTPUT_ON_ERROR;

    /** @var array<class-string, array<string, true>> */
    private static array $sensitiveParameterNames = [];

    public static function getClass(object $object): string
    {}

    /**
     * Strips the declaration site PHP appends to anonymous class names after a NUL byte
     *
     * Takes the name rather than the object because stack trace frames only carry the string.
     *
     * @param class-string $class
     */
    public static function getClassName(string $class): string
    {}

    /**
     * Returns the names of the constructor parameters of a class which are marked #[SensitiveParameter]
     *
     * The attribute can only target parameters, and PHP does not copy it over to the promoted
     * property, so the constructor signature is the only place it can be found. Non-promoted
     * parameters are included too, to be matched by name against the object's properties, as
     * assigning a sensitive parameter to a same-named property is just as common.
     *
     * Results are cached per class as the reflection cost is otherwise prohibitive.
     *
     * @param  class-string        $class
     * @return array<string, true> Set of parameter names, to be used with isset()
     */
    public static function getSensitiveParameterNames(string $class): array
    {}

    /**
     * Resolves a chain of formatter decorators down to the formatter doing the actual work
     *
     * Useful for handlers which only accept one specific formatter, to let them accept it
     * wrapped in a decorator like the RedactingFormatter.
     */
    public static function unwrapFormatter(FormatterInterface $formatter): FormatterInterface
    {}

    public static function substr(string $string, int $start, ?int $length = null): string
    {}

    /**
     * Makes sure if a relative path is passed in it is turned into an absolute path
     *
     * @param string $streamUrl stream URL or path without protocol
     */
    public static function canonicalizePath(string $streamUrl): string
    {}

    /**
     * Return the JSON representation of a value
     *
     * @param  mixed             $data
     * @param  int               $encodeFlags  flags to pass to json encode, defaults to DEFAULT_JSON_FLAGS
     * @param  bool              $ignoreErrors whether to ignore encoding errors or to throw on error, when ignored and the encoding fails, "null" is returned which is valid json for null
     * @throws \RuntimeException if encoding fails and errors are not ignored
     * @return string            when errors are ignored and the encoding fails, "null" is returned which is valid json for null
     */
    public static function jsonEncode($data, ?int $encodeFlags = null, bool $ignoreErrors = false): string
    {}

    /**
     * Handle a json_encode failure.
     *
     * If the failure is due to invalid string encoding, try to clean the
     * input and encode again. If the second encoding attempt fails, the
     * initial error is not encoding related or the input can't be cleaned then
     * raise a descriptive exception.
     *
     * @param  int               $code        return code of json_last_error function
     * @param  mixed             $data        data that was meant to be encoded
     * @param  int               $encodeFlags flags to pass to json encode, defaults to JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_PRESERVE_ZERO_FRACTION
     * @throws \RuntimeException if failure can't be corrected
     * @return string            JSON encoded data after error correction
     */
    public static function handleJsonError(int $code, $data, ?int $encodeFlags = null): string
    {}

    /**
     * Throws an exception according to a given code with a customized message
     *
     * @param  int               $code return code of json_last_error function
     * @param  mixed             $data data that was meant to be encoded
     * @throws \RuntimeException
     */
    private static function throwEncodeError(int $code, $data): never
    {}

    /**
     * Detect invalid UTF-8 string characters and convert to valid UTF-8.
     *
     * Valid UTF-8 input will be left unmodified, but strings containing
     * invalid UTF-8 codepoints will be reencoded as UTF-8 with an assumed
     * original encoding of ISO-8859-15. This conversion may result in
     * incorrect output if the actual encoding was not ISO-8859-15, but it
     * will be clean UTF-8 output and will not rely on expensive and fragile
     * detection algorithms.
     *
     * Function converts the input in place in the passed variable so that it
     * can be used as a callback for array_walk_recursive.
     *
     * @param mixed $data Input to check and convert if needed, passed by ref
     */
    private static function detectAndCleanUtf8(&$data): void
    {}

    /**
     * Converts a string with a valid 'memory_limit' format, to bytes.
     *
     * @param  string|false $val
     * @return int|false    Returns an integer representing bytes. Returns FALSE in case of error.
     */
    public static function expandIniShorthandBytes($val)
    {}

    public static function getRecordMessageForException(LogRecord $record): string
    {}
}
