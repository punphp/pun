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
use League\Uri\Contracts\Conditionable;
use League\Uri\Contracts\Transformable;
use League\Uri\Contracts\UriException;
use League\Uri\Contracts\UriInterface;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\UriTemplate\TemplateCanNotBeExpanded;
use Psr\Http\Message\UriInterface as Psr7UriInterface;
use Stringable;
use Uri\Rfc3986\Uri as Rfc3986Uri;
use Uri\WhatWg\Url as WhatWgUrl;

use function is_bool;
use function ltrim;

/**
 * @phpstan-import-type InputComponentMap from UriString
 */
final class Http implements Stringable, Psr7UriInterface, JsonSerializable, Conditionable, Transformable
{
    private readonly UriInterface $uri;

    private function __construct(UriInterface $uri)
    {}

    /**
     * PSR-7 UriInterface makes the following normalization.
     *
     * Safely stringify input when possible for League UriInterface compatibility.
     *
     * Query, Fragment and User Info when undefined are normalized to the empty string
     */
    private function normalizePsr7Uri(UriInterface $uri): UriInterface
    {}

    /**
     * Create a new instance from a string or a stringable object.
     */
    public static function new(Rfc3986Uri|WhatwgUrl|Stringable|string $uri = ''): self
    {}

    /**
     * Create a new instance from a string or a stringable structure or returns null on failure.
     */
    public static function tryNew(Rfc3986Uri|WhatwgUrl|Stringable|string $uri = ''): ?self
    {}

    /**
     * Create a new instance from a hash of parse_url parts.
     *
     * @param InputComponentMap $components a hash representation of the URI similar
     *                                      to PHP parse_url function result
     */
    public static function fromComponents(array $components): self
    {}

    /**
     * Create a new instance from the environment.
     */
    public static function fromServer(array $server): self
    {}

    /**
     * Creates a new instance from a template.
     *
     * @throws TemplateCanNotBeExpanded if the variables are invalid or missing
     * @throws UriException if the variables are invalid or missing
     */
    public static function fromTemplate(Stringable|string $template, iterable $variables = []): self
    {}

    /**
     * Returns a new instance from a URI and a Base URI.or null on failure.
     *
     * The returned URI must be absolute if a base URI is provided
     */
    public static function parse(WhatWgUrl|Rfc3986Uri|Stringable|string $uri, WhatWgUrl|Rfc3986Uri|Stringable|string|null $baseUri = null): ?self
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

    public function getQuery(): string
    {}

    public function getFragment(): string
    {}

    public function __toString(): string
    {}

    public function jsonSerialize(): string
    {}

    /**
     * Safely stringify input when possible for League UriInterface compatibility.
     */
    private function filterInput(string $str): ?string
    {}

    private function newInstance(UriInterface $uri): self
    {}

    public function when(callable|bool $condition, callable $onSuccess, ?callable $onFail = null): static
    {}

    public function transform(callable $callback): static
    {}

    public function withScheme(string $scheme): self
    {}

    public function withUserInfo(string $user, ?string $password = null): self
    {}

    public function withHost(string $host): self
    {}

    public function withPort(?int $port): self
    {}

    public function withPath(string $path): self
    {}

    public function withQuery(string $query): self
    {}

    public function withFragment(string $fragment): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.6.0
     * @codeCoverageIgnore
     * @see Http::parse()
     *
     * Create a new instance from a URI and a Base URI.
     *
     * The returned URI must be absolute.
     */
    #[Deprecated(message:'use League\Uri\Http::parse() instead', since:'league/uri:7.6.0')]
    public static function fromBaseUri(Rfc3986Uri|WhatwgUrl|Stringable|string $uri, Rfc3986Uri|WhatwgUrl|Stringable|string|null $baseUri = null): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Http::new()
     *
     * Create a new instance from a string.
     */
    #[Deprecated(message:'use League\Uri\Http::new() instead', since:'league/uri:7.0.0')]
    public static function createFromString(Stringable|string $uri = ''): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Http::fromComponents()
     *
     * Create a new instance from a hash of parse_url parts.
     *
     * @param InputComponentMap $components a hash representation of the URI similar
     *                                      to PHP parse_url function result
     */
    #[Deprecated(message:'use League\Uri\Http::fromComponents() instead', since:'league/uri:7.0.0')]
    public static function createFromComponents(array $components): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Http::fromServer()
     *
     * Create a new instance from the environment.
     */
    #[Deprecated(message:'use League\Uri\Http::fromServer() instead', since:'league/uri:7.0.0')]
    public static function createFromServer(array $server): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Http::new()
     *
     * Create a new instance from a URI object.
     */
    #[Deprecated(message:'use League\Uri\Http::new() instead', since:'league/uri:7.0.0')]
    public static function createFromUri(Psr7UriInterface|UriInterface $uri): self
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     * @see Http::fromBaseUri()
     *
     * Create a new instance from a URI and a Base URI.
     *
     * The returned URI must be absolute.
     */
    #[Deprecated(message:'use League\Uri\Http::fromBaseUri() instead', since:'league/uri:7.0.0')]
    public static function createFromBaseUri(Stringable|string $uri, Stringable|string|null $baseUri = null): self
    {}
}
