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
use finfo;
use League\Uri\Contracts\Conditionable;
use League\Uri\Contracts\FragmentDirective;
use League\Uri\Contracts\Transformable;
use League\Uri\Contracts\UriComponentInterface;
use League\Uri\Contracts\UriException;
use League\Uri\Contracts\UriInterface;
use League\Uri\Exceptions\MissingFeature;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\Idna\Converter as IdnaConverter;
use League\Uri\IPv4\Converter as IPv4Converter;
use League\Uri\IPv6\Converter as IPv6Converter;
use League\Uri\UriTemplate\TemplateCanNotBeExpanded;
use Psr\Http\Message\UriInterface as Psr7UriInterface;
use RuntimeException;
use SensitiveParameter;
use SplFileInfo;
use SplFileObject;
use Stringable;
use Throwable;
use TypeError;
use Uri\Rfc3986\Uri as Rfc3986Uri;
use Uri\WhatWg\Url as WhatWgUrl;

use function array_filter;
use function array_key_last;
use function array_map;
use function array_pop;
use function array_shift;
use function base64_decode;
use function base64_encode;
use function basename;
use function count;
use function dirname;
use function explode;
use function fclose;
use function feof;
use function file_get_contents;
use function filter_var;
use function fopen;
use function fread;
use function fwrite;
use function gettype;
use function implode;
use function in_array;
use function is_bool;
use function is_object;
use function is_resource;
use function is_string;
use function preg_match;
use function preg_replace;
use function preg_replace_callback;
use function rawurldecode;
use function rawurlencode;
use function restore_error_handler;
use function set_error_handler;
use function sprintf;
use function str_contains;
use function str_repeat;
use function str_replace;
use function str_starts_with;
use function strlen;
use function strpos;
use function strspn;
use function strtolower;
use function substr;
use function trim;

use const FILEINFO_MIME;
use const FILEINFO_MIME_TYPE;
use const FILTER_FLAG_IPV4;
use const FILTER_NULL_ON_FAILURE;
use const FILTER_VALIDATE_BOOLEAN;
use const FILTER_VALIDATE_EMAIL;
use const FILTER_VALIDATE_IP;

/**
 * @phpstan-import-type ComponentMap from UriString
 * @phpstan-import-type InputComponentMap from UriString
 */
final class Uri implements Conditionable, UriInterface, Transformable
{
    /**
     * RFC3986 invalid characters.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-2.2
     *
     * @var string
     */
    private const REGEXP_INVALID_CHARS = '/[\x00-\x1f\x7f]/';

    /**
     * RFC3986 IPvFuture host and port component.
     *
     * @var string
     */
    private const REGEXP_HOST_PORT = ',^(?<host>(\[.*]|[^:])*)(:(?<port>[^/?#]*))?$,x';

    /**
     * Regular expression pattern to for file URI.
     * <volume> contains the volume but not the volume separator.
     * The volume separator may be URL-encoded (`|` as `%7C`) by formatPath(),
     * so we account for that here.
     *
     * @var string
     */
    private const REGEXP_FILE_PATH = ',^(?<delim>/)?(?<volume>[a-zA-Z])(?:[:|\|]|%7C)(?<rest>.*)?,';

    /**
     * Mimetype regular expression pattern.
     *
     * @link https://tools.ietf.org/html/rfc2397
     *
     * @var string
     */
    private const REGEXP_MIMETYPE = ',^\w+/[-.\w]+(?:\+[-.\w]+)?$,';

    /**
     * Base64 content regular expression pattern.
     *
     * @link https://tools.ietf.org/html/rfc2397
     *
     * @var string
     */
    private const REGEXP_BINARY = ',(;|^)base64$,';

    /**
     * Windows filepath regular expression pattern.
     * <root> contains both the volume and volume separator.
     *
     * @var string
     */
    private const REGEXP_WINDOW_PATH = ',^(?<root>[a-zA-Z][:|\|]),';

    /**
     * Maximum number of cached items.
     *
     * @var int
     */
    private const MAXIMUM_CACHED_ITEMS = 100;

    /**
     * All ASCII letters sorted by typical frequency of occurrence.
     *
     * @var string
     */
    private const ASCII = "\x20\x65\x69\x61\x73\x6E\x74\x72\x6F\x6C\x75\x64\x5D\x5B\x63\x6D\x70\x27\x0A\x67\x7C\x68\x76\x2E\x66\x62\x2C\x3A\x3D\x2D\x71\x31\x30\x43\x32\x2A\x79\x78\x29\x28\x4C\x39\x41\x53\x2F\x50\x22\x45\x6A\x4D\x49\x6B\x33\x3E\x35\x54\x3C\x44\x34\x7D\x42\x7B\x38\x46\x77\x52\x36\x37\x55\x47\x4E\x3B\x4A\x7A\x56\x23\x48\x4F\x57\x5F\x26\x21\x4B\x3F\x58\x51\x25\x59\x5C\x09\x5A\x2B\x7E\x5E\x24\x40\x60\x7F\x00\x01\x02\x03\x04\x05\x06\x07\x08\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

    private readonly ?string $scheme;
    private readonly ?string $user;
    private readonly ?string $pass;
    private readonly ?string $userInfo;
    private readonly ?string $host;
    private readonly ?int $port;
    private readonly ?string $authority;
    private readonly string $path;
    private readonly ?string $query;
    private readonly ?string $fragment;
    private readonly string $uriAsciiString;
    private readonly string $uriUnicodeString;
    private readonly ?string $origin;

    private function __construct(
        ?string $scheme,
        ?string $user,
        #[SensitiveParameter] ?string $pass,
        ?string $host,
        ?int $port,
        string $path,
        ?string $query,
        ?string $fragment
    ) {}

    /**
     * Format the Scheme and Host component.
     *
     * @throws SyntaxError if the scheme is invalid
     */
    private function formatScheme(?string $scheme): ?string
    {}

    /**
     * Validate and Format the Host component.
     */
    private function formatHost(?string $host): ?string
    {}

    /**
     * Format the Port component.
     *
     * @throws SyntaxError
     */
    private function formatPort(BackedEnum|int|null $port = null): ?int
    {}

    /**
     * Create a new instance from a string or a stringable structure or returns null on failure.
     */
    public static function tryNew(Rfc3986Uri|WhatWgUrl|Urn|Stringable|string $uri = ''): ?self
    {}

    /**
     * Create a new instance from a string.
     */
    public static function new(Rfc3986Uri|WhatWgUrl|Urn|BackedEnum|Stringable|string $uri = ''): self
    {}

    /**
     * Returns a new instance from a URI and a Base URI.or null on failure.
     *
     * The returned URI must be absolute if a base URI is provided
     */
    public static function parse(Rfc3986Uri|WhatWgUrl|Urn|BackedEnum|Stringable|string $uri, Rfc3986Uri|WhatWgUrl|Urn|BackedEnum|Stringable|string|null $baseUri = null): ?self
    {}

    /**
     * Creates a new instance from a template.
     *
     * @throws TemplateCanNotBeExpanded if the variables are invalid or missing
     * @throws UriException if the resulting expansion cannot be converted to a UriInterface instance
     */
    public static function fromTemplate(BackedEnum|UriTemplate|Stringable|string $template, iterable $variables = []): self
    {}

    /**
     * Create a new instance from a hash representation of the URI similar
     * to PHP parse_url function result.
     *
     * @param InputComponentMap $components a hash representation of the URI similar to PHP parse_url function result
     */
    public static function fromComponents(array $components = []): self
    {}

    /**
     * Create a new instance from a data file path.
     *
     * @param SplFileInfo|SplFileObject|resource|Stringable|string $path
     * @param ?resource $context
     *
     * @throws MissingFeature If ext/fileinfo is not installed
     * @throws SyntaxError If the file does not exist or is not readable
     */
    public static function fromFileContents(mixed $path, $context = null): self
    {}

    /**
     * Create a new instance from a data URI string.
     *
     * @throws SyntaxError If the parameter syntax is invalid
     */
    public static function fromData(BackedEnum|Stringable|string $data, string $mimetype = '', string $parameters = ''): self
    {}

    /**
     * Create a new instance from a Unix path string.
     */
    public static function fromUnixPath(BackedEnum|Stringable|string $path): self
    {}

    /**
     * Create a new instance from a local Windows path string.
     */
    public static function fromWindowsPath(BackedEnum|Stringable|string $path): self
    {}

    /**
     * Creates a new instance from a RFC8089 compatible URI.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc8089
     */
    public static function fromRfc8089(BackedEnum|Stringable|string $uri): static
    {}

    /**
     * Create a new instance from the environment.
     */
    public static function fromServer(array $server): self
    {}

    /**
     * Returns the environment scheme.
     */
    private static function fetchScheme(array $server): string
    {}

    /**
     * Returns the environment user info.
     *
     * @return non-empty-array {0: ?string, 1: ?string}
     */
    private static function fetchUserInfo(array $server): array
    {}

    /**
     * Returns the environment host.
     *
     * @throws SyntaxError If the host cannot be detected
     *
     * @return array{0:string|null, 1:int|null}
     */
    private static function fetchHostname(array $server): array
    {}

    /**
     * Returns the environment path.
     *
     * @return list<?string>
     */
    private static function fetchRequestUri(array $server): array
    {}

    /**
     * Format the Path component.
     */
    private function formatPath(string $path): string
    {}

    /**
     * Filter the Path component.
     *
     * @link https://tools.ietf.org/html/rfc2397
     *
     * @throws SyntaxError If the path is not compliant with RFC2397
     */
    private static function formatDataPath(string $path): string
    {}

    /**
     * Assert the path is a compliant with RFC2397.
     *
     * @link https://tools.ietf.org/html/rfc2397
     *
     * @throws SyntaxError If the mediatype or the data are not compliant with the RFC2397
     */
    private static function assertValidPath(string $mimetype, string $parameters, string $data): void
    {}

    /**
     * Validate mediatype parameter.
     */
    private static function validateParameter(string $parameter): bool
    {}

    /**
     * Format the path component for the URI scheme file.
     */
    private static function formatFilePath(string $path): string
    {}

    /**
     * assert the URI internal state is valid.
     *
     * @link https://tools.ietf.org/html/rfc3986#section-3
     * @link https://tools.ietf.org/html/rfc3986#section-3.3
     *
     * @throws SyntaxError if the URI is in an invalid state, according to RFC3986
     */
    private function assertValidRfc3986Uri(): void
    {}

    /**
     * assert the URI scheme is valid.
     *
     * @link https://w3c.github.io/FileAPI/#url
     * @link https://datatracker.ietf.org/doc/html/rfc2397
     * @link https://tools.ietf.org/html/rfc3986#section-3
     * @link https://tools.ietf.org/html/rfc3986#section-3.3
     *
     * @throws SyntaxError if the URI is in an invalid state, according to scheme-specific rules
     */
    private function assertValidState(): void
    {}

    private function isValidBlob(): bool
    {}

    private function isValidMailto(): bool
    {}

    private static function validateEmailList(string $emails): bool
    {}

    /**
     * Sets the URI origin.
     *
     * The origin read-only property of the URL interface returns a string containing
     * the Unicode serialization of the represented URL.
     */
    private function setOrigin(): ?string
    {}

    /**
     * URI validation for URI schemes which allows only scheme and path components.
     */
    private function isUriWithSchemeAndPathOnly(): bool
    {}

    /**
     * URI validation for URI schemes which allows only scheme, host and path components.
     */
    private function isUriWithSchemeHostAndPathOnly(): bool
    {}

    /**
     * URI validation for URI schemes which disallow the empty '' host.
     */
    private function isNonEmptyHostUri(): bool
    {}

    /**
     * URI validation for URIs schemes which disallow the empty '' host
     * and forbids the fragment component.
     */
    private function isNonEmptyHostUriWithoutFragment(): bool
    {}

    /**
     * URI validation for URIs schemes which disallow the empty '' host
     * and forbids fragment and query components.
     */
    private function isNonEmptyHostUriWithoutFragmentAndQuery(): bool
    {}

    public function __toString(): string
    {}

    /**
     * Returns the string representation as a URI reference.
     *
     * @see http://tools.ietf.org/html/rfc3986#section-4.1
     * @see ::toString
     */
    public function jsonSerialize(): string
    {}

    /**
     * Returns the string representation as a URI reference.
     *
     * @see http://tools.ietf.org/html/rfc3986#section-4.1
     */
    public function toString(): string
    {}

    /**
     * Returns the string representation as a URI reference.
     *
     * @see http://tools.ietf.org/html/rfc3986#section-4.1
     */
    public function toAsciiString(): string
    {}

    /**
     * Returns the string representation as a URI reference.
     *
     * The host is converted to its UNICODE representation if available
     */
    public function toUnicodeString(): string
    {}

    /**
     * Returns the human-readable string representation of the URI as an IRI.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3987
     */
    public function toDisplayString(): string
    {}

    /**
     * Returns the Unix filesystem path.
     *
     * The method will return null if a scheme is present and is not the `file` scheme
     */
    public function toUnixPath(): ?string
    {}

    /**
     * Returns the Windows filesystem path.
     *
     * The method will return null if a scheme is present and is not the `file` scheme
     */
    public function toWindowsPath(): ?string
    {}

    /**
     * Returns a string representation of a File URI according to RFC8089.
     *
     * The method will return null if the URI scheme is not the `file` scheme
     *
     * @see https://datatracker.ietf.org/doc/html/rfc8089
     */
    public function toRfc8089(): ?string
    {}

    /**
     * Save the data to a specific file.
     *
     * The method returns the number of bytes written to the file
     * or null for any other scheme except the data scheme
     *
     * @param SplFileInfo|SplFileObject|resource|Stringable|string $destination
     * @param ?resource $context
     *
     * @throws RuntimeException if the content cannot be stored.
     */
    public function toFileContents(mixed $destination, $context = null): ?int
    {}

    /**
     * Returns an associative array containing all the URI components.
     *
     * @return ComponentMap
     */
    public function toComponents(): array
    {}

    public function getScheme(): ?string
    {}

    public function getAuthority(): ?string
    {}

    /**
     * Returns the user component encoded value.
     *
     * @see https://wiki.php.net/rfc/url_parsing_api
     */
    public function getUsername(): ?string
    {}

    public function getPassword(): ?string
    {}

    public function getUserInfo(): ?string
    {}

    public function getHost(): ?string
    {}

    public function getUnicodeHost(): ?string
    {}

    public function isIpv4Host(): bool
    {}

    public function isIpv6Host(): bool
    {}

    public function isIpvFutureHost(): bool
    {}

    public function isIpHost(): bool
    {}

    public function isRegisteredNameHost(): bool
    {}

    public function isDomainHost(): bool
    {}

    public function getPort(): ?int
    {}

    public function getPath(): string
    {}

    public function getQuery(): ?string
    {}

    public function getFragment(): ?string
    {}

    public function getOrigin(): ?string
    {}

    public function when(callable|bool $condition, callable $onSuccess, ?callable $onFail = null): static
    {}

    public function transform(callable $callback): static
    {}

    public function withScheme(BackedEnum|Stringable|string|null $scheme): static
    {}

    /**
     * Filter a string.
     *
     * @throws SyntaxError if the submitted data cannot be converted to string
     */
    private function filterString(BackedEnum|Stringable|string|null $str): ?string
    {}

    public function withUserInfo(
        BackedEnum|Stringable|string|null $user,
        #[SensitiveParameter] BackedEnum|Stringable|string|null $password = null
    ): static {}

    public function withUsername(BackedEnum|Stringable|string|null $user): static
    {}

    public function withPassword(#[SensitiveParameter] BackedEnum|Stringable|string|null $password): static
    {}

    public function withHost(BackedEnum|Stringable|string|null $host): static
    {}

    public function withPort(BackedEnum|int|null $port): static
    {}

    public function withPath(BackedEnum|Stringable|string $path): static
    {}

    public function withQuery(BackedEnum|Stringable|string|null $query): static
    {}

    public function withFragment(BackedEnum|Stringable|string|null $fragment): static
    {}

    /**
     * Tells whether the `file` scheme base URI represents a local file.
     */
    public function isLocalFile(): bool
    {}

    /**
     * Tells whether the URI is opaque or not.
     *
     * A URI is opaque if and only if it is absolute
     * and does not have an authority path.
     */
    public function isOpaque(): bool
    {}

    /**
     * Tells whether two URI do not share the same origin.
     */
    public function isCrossOrigin(Rfc3986Uri|WhatWgUrl|Urn|Stringable|string $uri): bool
    {}

    public function isSameOrigin(Rfc3986Uri|WhatWgUrl|Urn|Stringable|string $uri): bool
    {}

    /**
     * Tells whether the URI is absolute.
     */
    public function isAbsolute(): bool
    {}

    /**
     * Tells whether the URI is a network path.
     */
    public function isNetworkPath(): bool
    {}

    /**
     * Tells whether the URI is an absolute path.
     */
    public function isAbsolutePath(): bool
    {}

    /**
     * Tells whether the URI is a relative path.
     */
    public function isRelativePath(): bool
    {}

    /**
     * Tells whether both URIs refer to the same document.
     */
    public function isSameDocument(Rfc3986Uri|WhatWgUrl|UriInterface|Stringable|Urn|string $uri): bool
    {}

    public function equals(Rfc3986Uri|WhatWgUrl|UriInterface|Stringable|Urn|string $uri, UriComparisonMode $uriComparisonMode = UriComparisonMode::ExcludeFragment): bool
    {}

    /**
     * Normalize a URI by applying non-destructive and destructive normalization
     * rules as defined in RFC3986 and RFC3987.
     */
    public function normalize(): static
    {}

    /**
     * Resolves a URI against a base URI using RFC3986 rules.
     *
     * This method MUST retain the state of the submitted URI instance, and return
     * a URI instance of the same type that contains the applied modifications.
     *
     * This method MUST be transparent when dealing with errors and exceptions.
     * It MUST not alter or silence them apart from validating its own parameters.
     */
    public function resolve(Rfc3986Uri|WhatWgUrl|UriInterface|Stringable|Urn|BackedEnum|string $uri): static
    {}

    /**
     * Relativize a URI according to a base URI.
     *
     * This method MUST retain the state of the submitted URI instance, and return
     * a URI instance of the same type that contains the applied modifications.
     *
     * This method MUST be transparent when dealing with error and exceptions.
     * It MUST not alter of silence them apart from validating its own parameters.
     */
    public function relativize(Rfc3986Uri|WhatWgUrl|UriInterface|Stringable|Urn|BackedEnum|string $uri): static
    {}

    /**
     * Formatting the path to keep a resolvable URI.
     */
    private static function formatPathWithEmptyBaseQuery(string $path): string
    {}

    /**
     * Relatives the URI for an authority-less target URI.
     */
    private static function relativizePath(string $path, string $basePath): string
    {}

    /**
     * Formatting the path to keep a valid URI.
     */
    private static function formatRelativePath(string $path, string $basePath): string
    {}

    /**
     * returns the path segments.
     *
     * @return array<string>
     */
    private static function getSegments(string $path): array
    {}

    /**
     * @return ComponentMap
     */
    public function __debugInfo(): array
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.6.0
     * @codeCoverageIgnore
     * @see Uri::parse()
     *
     * Creates a new instance from a URI and a Base URI.
     *
     * The returned URI must be absolute.
     */
    #[Deprecated(message:'use League\Uri\Uri::parse() instead', since:'league/uri:7.6.0')]
    public static function fromBaseUri(WhatWgUrl|Rfc3986Uri|Stringable|string $uri, WhatWgUrl|Rfc3986Uri|Stringable|string|null $baseUri = null): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.5.0
     * @codeCoverageIgnore
     * @see Uri::toComponents()
     *
     * @return ComponentMap
     */
    #[Deprecated(message:'use League\Uri\Uri::toComponents() instead', since:'league/uri:7.5.0')]
    public function getComponents(): array
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::new()
     */
    #[Deprecated(message:'use League\Uri\Uri::new() instead', since:'league/uri:7.0.0')]
    public static function createFromString(Stringable|string $uri = ''): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::fromComponents()
     *
     * @param InputComponentMap $components a hash representation of the URI similar to PHP parse_url function result
     */
    #[Deprecated(message:'use League\Uri\Uri::fromComponents() instead', since:'league/uri:7.0.0')]
    public static function createFromComponents(array $components = []): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @param resource|null $context
     *
     * @throws MissingFeature If ext/fileinfo is not installed
     * @throws SyntaxError If the file does not exist or is not readable
     * @see Uri::fromFileContents()
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     */
    #[Deprecated(message:'use League\Uri\Uri::fromDataPath() instead', since:'league/uri:7.0.0')]
    public static function createFromDataPath(string $path, $context = null): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::fromBaseUri()
     *
     * Creates a new instance from a URI and a Base URI.
     *
     * The returned URI must be absolute.
     */
    #[Deprecated(message:'use League\Uri\Uri::fromBaseUri() instead', since:'league/uri:7.0.0')]
    public static function createFromBaseUri(
        Stringable|UriInterface|String $uri,
        Stringable|UriInterface|String|null $baseUri = null
    ): static {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::fromUnixPath()
     *
     * Create a new instance from a Unix path string.
     */
    #[Deprecated(message:'use League\Uri\Uri::fromUnixPath() instead', since:'league/uri:7.0.0')]
    public static function createFromUnixPath(string $uri = ''): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::fromWindowsPath()
     *
     * Create a new instance from a local Windows path string.
     */
    #[Deprecated(message:'use League\Uri\Uri::fromWindowsPath() instead', since:'league/uri:7.0.0')]
    public static function createFromWindowsPath(string $uri = ''): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::new()
     *
     * Create a new instance from a URI object.
     */
    #[Deprecated(message:'use League\Uri\Uri::new() instead', since:'league/uri:7.0.0')]
    public static function createFromUri(Psr7UriInterface|UriInterface $uri): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Uri::fromServer()
     *
     * Create a new instance from the environment.
     */
    #[Deprecated(message:'use League\Uri\Uri::fromServer() instead', since:'league/uri:7.0.0')]
    public static function createFromServer(array $server): self
    {}
}
