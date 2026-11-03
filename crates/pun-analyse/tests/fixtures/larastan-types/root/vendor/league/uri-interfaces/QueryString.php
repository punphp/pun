<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri;

use BackedEnum;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\KeyValuePair\Converter;
use ReflectionEnum;
use ReflectionException;
use SplObjectStorage;
use Stringable;
use TypeError;
use UnitEnum;
use ValueError;

use function array_is_list;
use function array_key_exists;
use function array_keys;
use function get_debug_type;
use function get_object_vars;
use function http_build_query;
use function implode;
use function is_array;
use function is_object;
use function is_resource;
use function is_scalar;
use function rawurldecode;
use function str_replace;
use function strpos;
use function substr;

use const PHP_QUERY_RFC1738;
use const PHP_QUERY_RFC3986;

/**
 * A class to parse the URI query string.
 *
 * @see https://tools.ietf.org/html/rfc3986#section-3.4
 */
final class QueryString
{
    private const PAIR_VALUE_DECODED = 1;
    private const PAIR_VALUE_PRESERVED = 2;
    private const RECURSION_MARKER = "\0__RECURSION_INTERNAL_MARKER__\0";

    /**
     * @codeCoverageIgnore
     */
    private function __construct()
    {}

    /**
     * Build a query string from a list of pairs.
     *
     * @see QueryString::buildFromPairs()
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-2.2
     *
     * @param iterable<array{0:string, 1:mixed}> $pairs
     * @param non-empty-string $separator
     *
     * @throws SyntaxError If the encoding type is invalid
     * @throws SyntaxError If a pair is invalid
     */
    public static function build(iterable $pairs, string $separator = '&', int $encType = PHP_QUERY_RFC3986, StringCoercionMode $coercionMode = StringCoercionMode::Native): ?string
    {}

    /**
     * Build a query string from a list of pairs.
     *
     * The method expects the return value from Query::parse to build
     * a valid query string. This method differs from PHP http_build_query as
     * it does not modify parameters keys.
     *
     *  If a reserved character is found in a URI component and
     *  no delimiting role is known for that character, then it must be
     *  interpreted as representing the data octet corresponding to that
     *  character's encoding in US-ASCII.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-2.2
     *
     * @param iterable<array{0:string, 1:mixed}> $pairs
     *
     * @throws SyntaxError If the encoding type is invalid
     * @throws SyntaxError If a pair is invalid
     */
    public static function buildFromPairs(iterable $pairs, ?Converter $converter = null, StringCoercionMode $coercionMode = StringCoercionMode::Native): ?string
    {}

    /**
     * Build a query string from an object or an array like http_build_query without discarding values.
     * The method differs from http_build_query for the following behavior:
     *
     *  - if a resource is used, a TypeError is thrown.
     *  - if a recursion is detected a ValueError is thrown
     *  - the method preserves value with `null` value (http_build_query) skip the key.
     *  - the method does not handle prefix usage
     *
     * @param array<array-key, mixed> $data
     * @param non-empty-string $separator
     *
     * @throws TypeError if a resource is found it the input array
     * @throws ValueError if a recursion is detected
     */
    public static function compose(
        array|object $data,
        string $separator = '&',
        int $encType = PHP_QUERY_RFC1738,
        QueryComposeMode $composeMode = QueryComposeMode::Native
    ): ?string {}

    public static function composeFromValue(
        array|object $data,
        ?Converter $converter = null,
        QueryComposeMode $composeMode = QueryComposeMode::Native,
    ): ?string {}

    /**
     * @param array<array-key, mixed>|object $data
     * @param SplObjectStorage<object, null> $seenObjects
     *
     * @throws TypeError if a resource is found it the input array
     * @throws ValueError if a recursion is detected
     * @throws ReflectionException if reflection is not possible on the Enum
     *
     * @return iterable<array{0: array-key, 1: string|int|float|bool|null}>
     */
    private static function composeRecursive(
        QueryComposeMode $composeMode,
        array|object $data,
        string|int $prefix = '',
        SplObjectStorage $seenObjects = new SplObjectStorage(),
    ): iterable { yield; }

    /**
     * Array recursion detection.
     * @see https://stackoverflow.com/questions/9042142/detecting-infinite-array-recursion-in-php
     */
    private static function hasCircularReference(array &$arr): bool
    {}

    /**
     * Parses the query string.
     *
     * The result depends on the query parsing mode
     *
     * @see QueryString::extractFromValue()
     *
     * @param non-empty-string $separator
     *
     * @throws SyntaxError
     */
    public static function extract(
        BackedEnum|Stringable|string|bool|null $query,
        string $separator = '&',
        int $encType = PHP_QUERY_RFC3986,
        QueryExtractMode $extractMode = QueryExtractMode::Unmangled,
    ): array {}

    /**
     * Parses the query string.
     *
     * The result depends on the query parsing mode
     *
     * @throws SyntaxError
     */
    public static function extractFromValue(
        BackedEnum|Stringable|string|bool|null $query,
        ?Converter $converter = null,
        QueryExtractMode $extractMode = QueryExtractMode::Unmangled,
    ): array {}

    /**
     * Parses a query string into a collection of key/value pairs.
     *
     * @param non-empty-string $separator
     *
     * @throws SyntaxError
     *
     * @return array<int, array{0:string, 1:string|null}>
     */
    public static function parse(BackedEnum|Stringable|string|bool|null $query, string $separator = '&', int $encType = PHP_QUERY_RFC3986): array
    {}

    /**
     * Parses a query string into a collection of key/value pairs.
     *
     * @throws SyntaxError
     *
     * @return array<int, array{0:string, 1:string|null}>
     */
    public static function parseFromValue(BackedEnum|Stringable|string|bool|null $query, ?Converter $converter = null): array
    {}

    /**
     * @param list<non-empty-array<string|null>> $pairs
     *
     * @return array<int, array{0:string, 1:string|null}>
     */
    private static function decodePairs(array $pairs, int $pairValueState): array
    {}

    /**
     * Converts a collection of key/value pairs and returns
     * the store PHP variables as elements of an array.
     */
    public static function convert(iterable $pairs, QueryExtractMode $extractMode = QueryExtractMode::Unmangled): array
    {}

    /**
     * Parses a query pair like parse_str without mangling the results array keys.
     *
     * <ul>
     * <li>empty name are not saved</li>
     * <li>If the value from name is duplicated its corresponding value will be overwritten</li>
     * <li>if no "[" is detected the value is added to the return array with the name as index</li>
     * <li>if no "]" is detected after detecting a "[" the value is added to the return array with the name as index</li>
     * <li>if there's a mismatch in bracket usage the remaining part is dropped</li>
     * <li>“.” and “ ” are not converted to “_”</li>
     * <li>If there is no “]”, then the first “[” is not converted to becomes an “_”</li>
     * <li>no whitespace trimming is done on the key value</li>
     * </ul>
     *
     * @see https://php.net/parse_str
     * @see https://wiki.php.net/rfc/on_demand_name_mangling
     * @see https://github.com/php/php-src/blob/master/ext/standard/tests/strings/parse_str_basic1.phpt
     * @see https://github.com/php/php-src/blob/master/ext/standard/tests/strings/parse_str_basic2.phpt
     * @see https://github.com/php/php-src/blob/master/ext/standard/tests/strings/parse_str_basic3.phpt
     * @see https://github.com/php/php-src/blob/master/ext/standard/tests/strings/parse_str_basic4.phpt
     *
     * @param array $data the submitted array
     * @param array|string $name the pair key
     * @param string $value the pair value
     */
    private static function extractPhpVariable(
        array $data,
        array|string $name,
        ?string $value = '',
        QueryExtractMode $extractMode = QueryExtractMode::Unmangled
    ): array {}
}
