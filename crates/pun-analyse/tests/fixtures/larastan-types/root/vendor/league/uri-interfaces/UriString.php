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
use Deprecated;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\Idna\Converter as IdnaConverter;
use Stringable;
use Throwable;

use function array_map;
use function array_merge;
use function array_pop;
use function array_reduce;
use function defined;
use function explode;
use function filter_var;
use function function_exists;
use function implode;
use function ltrim;
use function preg_match;
use function sprintf;
use function str_replace;
use function strpos;
use function strtolower;
use function substr;

use const FILTER_FLAG_IPV4;
use const FILTER_VALIDATE_INT;
use const FILTER_VALIDATE_IP;

/**
 * A class to parse a URI string according to RFC3986.
 *
 * @link    https://tools.ietf.org/html/rfc3986
 * @package League\Uri
 * @author  Ignace Nyamagana Butera <nyamsprod@gmail.com>
 * @since   6.0.0
 *
 * @phpstan-type AuthorityMap array{user: ?string, pass: ?string, host: ?string, port: ?int}
 * @phpstan-type ComponentMap array{scheme: ?string, user: ?string, pass: ?string, host: ?string, port: ?int, path: string, query: ?string, fragment: ?string}
 * @phpstan-type InputComponentMap array{scheme? : ?string, user? : ?string, pass? : ?string, host? : ?string, port? : ?int, path? : ?string, query? : ?string, fragment? : ?string}
 */
final class UriString
{
    /**
     * Default URI component values.
     *
     * @var ComponentMap
     */
    private const URI_COMPONENTS = [
        'scheme' => null, 'user' => null, 'pass' => null, 'host' => null,
        'port' => null, 'path' => '', 'query' => null, 'fragment' => null,
    ];

    /**
     * Simple URI which do not need any parsing.
     *
     * @var array<string, array<string>>
     */
    private const URI_SHORTCUTS = [
        '' => ['path' => ''],
        '#' => ['fragment' => ''],
        '?' => ['query' => ''],
        '?#' => ['query' => '', 'fragment' => ''],
        '/' => ['path' => '/'],
        '//' => ['host' => ''],
        '///' => ['host' => '', 'path' => '/'],
    ];

    /**
     * Range of invalid characters in URI 3986 string.
     *
     * @var string
     */
    private const REGEXP_VALID_URI_RFC3986_CHARS = '/^(?:[A-Za-z0-9\-._~:\/?#[\]@!$&\'()*+,;=%]|%[0-9A-Fa-f]{2})*$/';

    /**
     * Range of invalid characters in URI 3987 string.
     *
     * @var string
     */
    private const REGEXP_INVALID_URI_RFC3987_CHARS = '/[\x00-\x1f\x7f\s]/';

    /**
     * RFC3986 regular expression URI splitter.
     *
     * @link https://tools.ietf.org/html/rfc3986#appendix-B
     * @var string
     */
    private const REGEXP_URI_PARTS = ',^
        (?<scheme>(?<scontent>[^:/?\#]+):)?    # URI scheme component
        (?<authority>//(?<acontent>[^/?\#]*))? # URI authority part
        (?<path>[^?\#]*)                       # URI path component
        (?<query>\?(?<qcontent>[^\#]*))?       # URI query component
        (?<fragment>\#(?<fcontent>.*))?        # URI fragment component
    ,x';

    /**
     * URI scheme regular expression.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3.1
     * @var string
     */
    private const REGEXP_URI_SCHEME = '/^([a-z][a-z\d+.-]*)?$/i';

    /**
     * Invalid path for URI without scheme and authority regular expression.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3.3
     * @var string
     */
    private const REGEXP_INVALID_PATH = ',^(([^/]*):)(.*)?/,';

    /**
     * Host and Port splitter regular expression.
     *
     * @var string
     */
    private const REGEXP_HOST_PORT = ',^(?<host>\[.*\]|[^:]*)(:(?<port>.*))?$,';

    /** @var array<string,int> */
    private const DOT_SEGMENTS = ['.' => 1, '..' => 1];

    /**
     * Generate an IRI string representation (RFC3987) from its parsed representation
     * returned by League\UriString::parse() or PHP's parse_url.
     *
     * If you supply your own array, you are responsible for providing
     * valid components without their URI delimiters.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-5.3
     * @link https://tools.ietf.org/html/rfc3986#section-7.5
     */
    public static function toIriString(BackedEnum|Stringable|string $uri): string
    {}

    /**
     * Generate a URI string representation from its parsed representation
     * returned by League\UriString::parse() or PHP's parse_url.
     *
     * If you supply your own array, you are responsible for providing
     * valid components without their URI delimiters.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-5.3
     * @link https://tools.ietf.org/html/rfc3986#section-7.5
     *
     * @param InputComponentMap $components
     */
    public static function build(array $components): string
    {}

    /**
     * Generates a URI string representation based on RFC3986 algorithm.
     *
     * Valid URI component MUST be provided without their URI delimiters
     * but properly encoded.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-5.3
     * @link https://tools.ietf.org/html/rfc3986#section-7.5§
     */
    public static function buildUri(
        ?string $scheme = null,
        ?string $authority = null,
        ?string $path = null,
        ?string $query = null,
        ?string $fragment = null,
    ): string {}

    /**
     * Generate a URI authority representation from its parsed representation.
     *
     * @param InputComponentMap $components
     */
    public static function buildAuthority(array $components): ?string
    {}

    /**
     * Parses and normalizes the URI following RFC3986 destructive and non-destructive constraints.
     *
     * @throws SyntaxError if the URI is not parsable
     *
     * @return ComponentMap
     */
    public static function parseNormalized(Stringable|string $uri): array
    {}

    /**
     * Parses and normalizes the URI following RFC3986 destructive and non-destructive constraints.
     *
     * @throws SyntaxError if the URI is not parsable
     */
    public static function normalize(Stringable|string $uri): string
    {}

    /**
     * Parses and normalizes the URI following RFC3986 destructive and non-destructive constraints.
     *
     * @throws SyntaxError if the URI is not parsable
     */
    public static function normalizeAuthority(Stringable|string|null $authority): ?string
    {}

    /**
     * Resolves a URI against a base URI using RFC3986 rules.
     *
     * This method MUST retain the state of the submitted URI instance, and return
     * a URI instance of the same type that contains the applied modifications.
     *
     * This method MUST be transparent when dealing with error and exceptions.
     * It MUST not alter or silence them apart from validating its own parameters.
     *
     * @see https://www.rfc-editor.org/rfc/rfc3986.html#section-5
     *
     * @throws SyntaxError if the BaseUri is not absolute or in absence of a BaseUri if the uri is not absolute
     */
    public static function resolve(BackedEnum|Stringable|string $uri, BackedEnum|Stringable|string|null $baseUri = null): string
    {}

    /**
     * Filter Dot segment according to RFC3986.
     *
     * @see http://tools.ietf.org/html/rfc3986#section-5.2.4
     */
    public static function removeDotSegments(Stringable|string $path): string
    {}

    /**
     * Resolves an URI path and query component.
     *
     * @param ComponentMap $uri
     * @param ComponentMap $baseUri
     *
     * @return array{0:string, 1:string|null}
     */
    private static function resolvePathAndQuery(array $uri, array $baseUri): array
    {}

    public static function containsRfc3986Chars(Stringable|string $uri): bool
    {}

    public static function containsRfc3987Chars(Stringable|string $uri): bool
    {}

    /**
     * Parse a URI string into its components.
     *
     * This method parses a URI and returns an associative array containing any
     * of the various components of the URI that are present.
     *
     * <code>
     * $components = UriString::parse('http://foo@test.example.com:42?query#');
     * var_export($components);
     * //will display
     * array(
     *   'scheme' => 'http',           // the URI scheme component
     *   'user' => 'foo',              // the URI user component
     *   'pass' => null,               // the URI pass component
     *   'host' => 'test.example.com', // the URI host component
     *   'port' => 42,                 // the URI port component
     *   'path' => '',                 // the URI path component
     *   'query' => 'query',           // the URI query component
     *   'fragment' => '',             // the URI fragment component
     * );
     * </code>
     *
     * The returned array is similar to PHP's parse_url return value with the following
     * differences:
     *
     * <ul>
     * <li>All components are always present in the returned array</li>
     * <li>Empty and undefined component are treated differently. And empty component is
     *   set to the empty string while an undefined component is set to the `null` value.</li>
     * <li>The path component is never undefined</li>
     * <li>The method parses the URI following the RFC3986 rules, but you are still
     *   required to validate the returned components against its related scheme specific rules.</li>
     * </ul>
     *
     * @link https://tools.ietf.org/html/rfc3986
     *
     * @throws SyntaxError if the URI contains invalid characters
     * @throws SyntaxError if the URI contains an invalid scheme
     * @throws SyntaxError if the URI contains an invalid path
     *
     * @return ComponentMap
     */
    public static function parse(BackedEnum|Stringable|string|int $uri): array
    {}

    /**
     * Assert the URI internal state is valid.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3
     * @link https://tools.ietf.org/html/rfc3986#section-3.3
     *
     * @throws SyntaxError
     */
    private static function validateComponents(?string $scheme, ?string $authority, ?string $path): void
    {}

    /**
     * Parses the URI authority part.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3.2
     *
     * @throws SyntaxError If the port component is invalid
     *
     * @return AuthorityMap
     */
    public static function parseAuthority(BackedEnum|Stringable|string|null $authority): array
    {}

    /**
     * Filter and format the port component.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3.2.2
     *
     * @throws SyntaxError if the registered name is invalid
     */
    private static function filterPort(string $port): ?int
    {}

    /**
     * Returns whether a hostname is valid.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3.2.2
     *
     * @throws SyntaxError if the registered name is invalid
     */
    private static function filterHost(Stringable|string|null $host): ?string
    {}

    /**
     * Tells whether the scheme component is valid.
     */
    public static function isValidScheme(BackedEnum|Stringable|string|null $scheme): bool
    {}

    private static function normalizeHost(BackedEnum|Stringable|string|null $host): ?string
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.6.0
     * @codeCoverageIgnore
     * @see HostRecoord::validate()
     *
     * Create a new instance from the environment.
     */
    #[Deprecated(message:'use League\Uri\HostRecord::validate() instead', since:'league/uri:7.6.0')]
    public static function isValidHost(Stringable|string|null $host): bool
    {}
}
