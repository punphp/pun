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

use Deprecated;
use JsonSerializable;
use League\Uri\Contracts\UriAccess;
use League\Uri\Contracts\UriInterface;
use League\Uri\Exceptions\MissingFeature;
use League\Uri\Idna\Converter as IdnaConverter;
use League\Uri\IPv4\Converter as IPv4Converter;
use League\Uri\IPv6\Converter as IPv6Converter;
use Psr\Http\Message\UriFactoryInterface;
use Psr\Http\Message\UriInterface as Psr7UriInterface;
use Stringable;

use function array_pop;
use function array_reduce;
use function count;
use function explode;
use function implode;
use function in_array;
use function preg_match;
use function rawurldecode;
use function sort;
use function str_contains;
use function str_repeat;
use function str_replace;
use function strpos;
use function substr;

/**
 * @phpstan-import-type ComponentMap from UriInterface
 * @deprecated since version 7.6.0
 *
 * @see Modifier
 * @see Uri
 */
class BaseUri implements Stringable, JsonSerializable, UriAccess
{
    /** @var array<string,int> */
    final protected const WHATWG_SPECIAL_SCHEMES = ['ftp' => 1, 'http' => 1, 'https' => 1, 'ws' => 1, 'wss' => 1];

    /** @var array<string,int> */
    final protected const DOT_SEGMENTS = ['.' => 1, '..' => 1];

    protected readonly Psr7UriInterface|UriInterface|null $origin;
    protected readonly ?string $nullValue;

    /**
     * @param UriFactoryInterface|null $uriFactory Deprecated, will be removed in the next major release
     */
    final protected function __construct(
        protected readonly Psr7UriInterface|UriInterface $uri,
        protected readonly ?UriFactoryInterface $uriFactory
    ) {}

    public static function from(Stringable|string $uri, ?UriFactoryInterface $uriFactory = null): static
    {}

    public function withUriFactory(UriFactoryInterface $uriFactory): static
    {}

    public function withoutUriFactory(): static
    {}

    public function getUri(): Psr7UriInterface|UriInterface
    {}

    public function getUriString(): string
    {}

    public function jsonSerialize(): string
    {}

    public function __toString(): string
    {}

    public function origin(): ?self
    {}

    /**
     * Returns the Unix filesystem path.
     *
     * The method will return null if a scheme is present and is not the `file` scheme
     */
    public function unixPath(): ?string
    {}

    /**
     * Returns the Windows filesystem path.
     *
     * The method will return null if a scheme is present and is not the `file` scheme
     */
    public function windowsPath(): ?string
    {}

    /**
     * Returns a string representation of a File URI according to RFC8089.
     *
     * The method will return null if the URI scheme is not the `file` scheme
     */
    public function toRfc8089(): ?string
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
    public function isCrossOrigin(Stringable|string $uri): bool
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
     * Tells whether both URI refers to the same document.
     */
    public function isSameDocument(Stringable|string $uri): bool
    {}

    private static function normalizedUri(Stringable|string $uri): Uri
    {}

    /**
     * Tells whether the URI contains an Internationalized Domain Name (IDN).
     */
    public function hasIdn(): bool
    {}

    /**
     * Tells whether the URI contains an IPv4 regardless if it is mapped or native.
     */
    public function hasIPv4(): bool
    {}

    /**
     * Resolves a URI against a base URI using RFC3986 rules.
     *
     * This method MUST retain the state of the submitted URI instance, and return
     * a URI instance of the same type that contains the applied modifications.
     *
     * This method MUST be transparent when dealing with error and exceptions.
     * It MUST not alter or silence them apart from validating its own parameters.
     */
    public function resolve(Stringable|string $uri): static
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
    public function relativize(Stringable|string $uri): static
    {}

    final protected function computeOrigin(Psr7UriInterface|UriInterface $uri, ?string $nullValue): Psr7UriInterface|UriInterface|null
    {}

    /**
     * Input URI normalization to allow Stringable and string URI.
     */
    final protected static function filterUri(Stringable|string $uri, UriFactoryInterface|null $uriFactory = null): Psr7UriInterface|UriInterface
    {}

    /**
     * Tells whether the component value from both URI object equals.
     *
     * @pqram 'query'|'authority'|'scheme' $property
     */
    final protected function componentEquals(string $property, Psr7UriInterface|UriInterface $uri): bool
    {}

    /**
     * Filter the URI object.
     */
    final protected static function formatHost(Psr7UriInterface|UriInterface $uri): Psr7UriInterface|UriInterface
    {}

    /**
     * Tells whether the submitted URI object can be relativized.
     */
    final protected function canNotBeRelativize(Psr7UriInterface|UriInterface $uri): bool
    {}

    /**
     * Relatives the URI for an authority-less target URI.
     */
    final protected static function relativizePath(string $path, string $basePath): string
    {}

    /**
     * returns the path segments.
     *
     * @return string[]
     */
    final protected static function getSegments(string $path): array
    {}

    /**
     * Formatting the path to keep a valid URI.
     */
    final protected static function formatPath(string $path, string $basePath): string
    {}

    /**
     * Formatting the path to keep a resolvable URI.
     */
    final protected static function formatPathWithEmptyBaseQuery(string $path): string
    {}

    /**
     * Normalizes a URI for comparison; this URI string representation is not suitable for usage as per RFC guidelines.
     *
     * @deprecated since version 7.6.0
     *
     * @codeCoverageIgnore
     */
    #[Deprecated(message:'no longer used by the isSameDocument method', since:'league/uri-interfaces:7.6.0')]
    final protected function normalize(Psr7UriInterface|UriInterface $uri): string
    {}


    /**
     * Remove dot segments from the URI path as per RFC specification.
     *
     * @deprecated since version 7.6.0
     *
     * @codeCoverageIgnore
     */
    #[Deprecated(message:'no longer used by the isSameDocument method', since:'league/uri-interfaces:7.6.0')]
    final protected function removeDotSegments(string $path): string
    {}

    /**
     * Resolves an URI path and query component.
     *
     * @return array{0:string, 1:string|null}
     *
     * @deprecated since version 7.6.0
     *
     * @codeCoverageIgnore
     */
    #[Deprecated(message:'no longer used by the isSameDocument method', since:'league/uri-interfaces:7.6.0')]
    final protected function resolvePathAndQuery(Psr7UriInterface|UriInterface $uri): array
    {}
}
