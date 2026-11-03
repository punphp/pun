<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use GuzzleHttp\Psr7\Exception\MalformedUriException;
use Psr\Http\Message\UriInterface;

/**
 * PSR-7 URI implementation.
 *
 * @author Michael Dowling
 * @author Tobias Schultze
 * @author Matthew Weier O'Phinney
 */
class Uri implements UriInterface, \JsonSerializable
{
    /**
     * Absolute http and https URIs require a host per RFC 9110 Section 4.2.1
     * but in generic URIs the host can be empty. So for http(s) URIs we apply
     * this default host when no host is given yet to form a valid URI.
     */
    private const HTTP_DEFAULT_HOST = 'localhost';

    private const DEFAULT_PORTS = [
        'http' => 80,
        'https' => 443,
        'ftp' => 21,
        'gopher' => 70,
        'nntp' => 119,
        'news' => 119,
        'telnet' => 23,
        'tn3270' => 23,
        'imap' => 143,
        'pop' => 110,
        'ldap' => 389,
        'ws' => 80,
        'wss' => 443,
    ];

    private const QUERY_SEPARATORS_REPLACEMENT = ['=' => '%3D', '&' => '%26', '+' => '%2B'];

    /** @var string Uri scheme. */
    private string $scheme = '';

    /** @var string Uri user info. */
    private string $userInfo = '';

    /** @var string Uri host. */
    private string $host = '';

    /** @var int|null Uri port. */
    private ?int $port = null;

    /** @var string Uri path. */
    private string $path = '';

    /** @var string Uri query string. */
    private string $query = '';

    /** @var string Uri fragment. */
    private string $fragment = '';

    public function __construct(
        #[\SensitiveParameter]
        string $uri = ''
    ) {}

    public function __toString(): string
    {}

    /**
     * Composes a URI reference string from its various components according to
     * RFC 3986 Section 5.3.
     *
     * Usually this method does not need to be called manually but instead is
     * used indirectly via `Psr\Http\Message\UriInterface::__toString`.
     *
     * PSR-7 UriInterface treats an empty component the same as a missing
     * component as `getQuery()`, `getFragment()` etc. always return a string.
     * This explains the slight difference to RFC 3986 Section 5.3.
     *
     * Another adjustment is that the authority separator is added even when the
     * authority is missing/empty for the "file" scheme. This is because PHP
     * stream functions like `file_get_contents` only work with `file:///myfile`
     * but not with `file:/myfile` although they are equivalent according to RFC
     * 3986. But `file:///` is the more common syntax for the file scheme anyway
     * (Chrome for example redirects to that format). The separator is omitted
     * when such a URI has a rootless or empty path: adding it would turn the
     * first path segment into the authority of the composed URI, or compose the
     * string `file://`, which cannot be parsed back into a URI.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-5.3
     */
    public static function composeComponents(?string $scheme, ?string $authority, string $path, ?string $query, ?string $fragment): string
    {}

    /**
     * Whether the URI has the default port of the current scheme.
     *
     * `Psr\Http\Message\UriInterface::getPort` may return null or the standard
     * port. This method can be used independently of the implementation.
     */
    public static function isDefaultPort(UriInterface $uri): bool
    {}

    /**
     * Whether the URI is absolute, i.e. it has a scheme.
     *
     * An instance of UriInterface can either be an absolute URI or a relative
     * reference. An absolute URI has a scheme. A relative reference is used to
     * express a URI relative to another URI, the base URI. Relative references
     * can be divided into several forms according to RFC 3986 Section 4.2:
     * - network-path references, e.g. `//example.com/path`
     * - absolute-path references, e.g. `/path`
     * - relative-path references, e.g. `subpath`
     *
     * @see Uri::isNetworkPathReference
     * @see Uri::isAbsolutePathReference
     * @see Uri::isRelativePathReference
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
     */
    public static function isAbsolute(UriInterface $uri): bool
    {}

    /**
     * Whether the URI is a network-path reference.
     *
     * A relative reference that begins with two slash characters is termed a
     * network-path reference.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
     */
    public static function isNetworkPathReference(UriInterface $uri): bool
    {}

    /**
     * Whether the URI is an absolute-path reference.
     *
     * A relative reference that begins with a single slash character is termed
     * an absolute-path reference.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
     */
    public static function isAbsolutePathReference(UriInterface $uri): bool
    {}

    /**
     * Whether the URI is a relative-path reference.
     *
     * A relative reference that does not begin with a slash character is termed
     * a relative-path reference.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
     */
    public static function isRelativePathReference(UriInterface $uri): bool
    {}

    /**
     * Whether the URI is a same-document reference.
     *
     * A same-document reference refers to a URI that is, aside from its
     * fragment component, identical to the base URI. When no base URI is given,
     * only an empty URI reference (apart from its fragment) is considered a
     * same-document reference.
     *
     * @param UriInterface      $uri  The URI to check
     * @param UriInterface|null $base An optional base URI to compare against
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.4
     */
    public static function isSameDocumentReference(UriInterface $uri, ?UriInterface $base = null): bool
    {}

    /**
     * Creates a new URI with a specific query string value removed.
     *
     * Any existing query string values that exactly match the provided key are
     * removed.
     *
     * @param UriInterface $uri URI to use as a base.
     * @param string       $key Query string key to remove.
     */
    public static function withoutQueryValue(UriInterface $uri, string $key): UriInterface
    {}

    /**
     * Creates a new URI with a specific query string value.
     *
     * Any existing query string values that exactly match the provided key are
     * removed and replaced with the given key value pair. A value of null will
     * set the query string key without a value, e.g. "key" instead of
     * "key=value".
     *
     * @param UriInterface $uri   URI to use as a base.
     * @param string       $key   Key to set.
     * @param string|null  $value Value to set
     */
    public static function withQueryValue(UriInterface $uri, string $key, ?string $value): UriInterface
    {}

    /**
     * Creates a new URI with multiple query string values.
     *
     * It has the same behavior as `withQueryValue()` but for an associative
     * array of key => value.
     *
     * @param UriInterface    $uri           URI to use as a base.
     * @param (string|null)[] $keyValueArray Associative array of key and values
     */
    public static function withQueryValues(UriInterface $uri, array $keyValueArray): UriInterface
    {}

    /**
     * @param mixed $value
     */
    private static function assertStringOrNullQueryValue($value): void
    {}

    /**
     * Creates a URI from a hash of `parse_url` components.
     *
     * @see https://www.php.net/manual/en/function.parse-url.php
     *
     * @throws MalformedUriException If the components do not form a valid URI.
     */
    public static function fromParts(
        #[\SensitiveParameter]
        array $parts
    ): UriInterface {}

    /**
     * @throws \InvalidArgumentException If the host is invalid.
     *
     * @internal
     */
    public static function assertValidHost(string $host): void
    {}

    public function getScheme(): string
    {}

    public function getAuthority(): string
    {}

    public function getUserInfo(): string
    {}

    public function getHost(): string
    {}

    public function getPort(): ?int
    {}

    public function getPath(): string
    {}

    /**
     * Returns the path as it appears within a URI's string form.
     *
     * getPath() collapses multiple leading slashes so that a path used in
     * isolation cannot be mistaken for a protocol-relative URL. Whole-URI
     * operations like reference resolution and normalization (RFC 3986
     * Sections 5 and 6) are defined on the URI string form, where the path
     * stays verbatim, so they must read the path through this method instead.
     * For direct instances of this class the path is derived from the stored
     * components, including the leading slash the string form adds to a
     * rootless path when an authority is present; for subclasses and other
     * implementations the path is split from the string form per RFC 3986
     * Appendix B, without validating or decoding any other component. The
     * scheme is only split off when the instance reports one, as a relative
     * reference can begin with a segment containing a colon that the Appendix
     * B expression would otherwise read as a scheme.
     *
     * @throws \RuntimeException If the path cannot be split from the string form.
     *
     * @internal
     */
    public static function rawPath(UriInterface $uri): string
    {}

    public function getQuery(): string
    {}

    public function getFragment(): string
    {}

    public function withScheme(string $scheme): UriInterface
    {}

    public function withUserInfo(
        string $user,
        #[\SensitiveParameter]
        ?string $password = null
    ): UriInterface {}

    public function withHost(string $host): UriInterface
    {}

    public function withPort(?int $port): UriInterface
    {}

    public function withPath(string $path): UriInterface
    {}

    public function withQuery(string $query): UriInterface
    {}

    public function withFragment(string $fragment): UriInterface
    {}

    public function jsonSerialize(): string
    {}

    /**
     * Apply parse_url parts to a URI.
     *
     * @param array $parts Array of parse_url parts to apply.
     */
    private function applyParts(
        #[\SensitiveParameter]
        array $parts
    ): void {}

    /**
     * @throws \InvalidArgumentException If the scheme is invalid.
     */
    private function filterScheme(string $scheme): string
    {}

    /**
     * @throws \InvalidArgumentException If the user info is invalid.
     */
    private function filterUserInfoComponent(
        #[\SensitiveParameter]
        string $component
    ): string {}

    /**
     * @throws \InvalidArgumentException If the host is invalid.
     */
    private function filterHost(string $host): string
    {}

    /**
     * @throws \InvalidArgumentException If the port is invalid.
     */
    private function filterPort(?int $port): ?int
    {}

    /**
     * @param mixed $port
     *
     * @throws \InvalidArgumentException If the port is invalid.
     */
    private function filterPortPart($port): ?int
    {}

    /**
     * @param mixed $port
     */
    private static function describeInvalidPort($port): string
    {}

    /**
     * @param (string|int)[] $keys
     *
     * @return string[]
     */
    private static function getFilteredQueryString(UriInterface $uri, array $keys): array
    {}

    private static function generateQueryString(string $key, ?string $value): string
    {}

    private function removeDefaultPort(): void
    {}

    /**
     * Filters the path of a URI
     *
     * @throws \InvalidArgumentException If the path is invalid.
     */
    private function filterPath(string $path): string
    {}

    /**
     * Filters the query string or fragment of a URI.
     *
     * @throws \InvalidArgumentException If the query or fragment is invalid.
     */
    private function filterQueryAndFragment(string $str): string
    {}

    private function filterComponent(
        string $pattern,
        #[\SensitiveParameter]
        string $component,
        string $context
    ): string {}

    private function rawurlencodeMatchZero(array $match): string
    {}

    private function validateState(): void
    {}
}
