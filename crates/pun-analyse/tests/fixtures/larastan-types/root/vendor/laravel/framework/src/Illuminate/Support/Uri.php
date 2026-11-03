<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Contracts\Routing\UrlRoutable;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\Support\Responsable;
use Illuminate\Http\RedirectResponse;
use Illuminate\Support\Stringable as SupportStringable;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Dumpable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use JsonSerializable;
use League\Uri\Contracts\UriInterface;
use League\Uri\Uri as LeagueUri;
use SensitiveParameter;
use Stringable;

class Uri implements Htmlable, JsonSerializable, Responsable, Stringable
{
    use Conditionable, Dumpable, Macroable, Tappable;

    /**
     * The URI instance.
     */
    protected UriInterface $uri;

    /**
     * The URL generator resolver.
     */
    protected static ?Closure $urlGeneratorResolver = null;

    /**
     * Create a new parsed URI instance.
     */
    public function __construct(UriInterface|Stringable|string $uri = '')
    {}

    /**
     * Create a new URI instance.
     */
    public static function of(UriInterface|Stringable|string $uri = ''): static
    {}

    /**
     * Get a URI instance of an absolute URL for the given path.
     */
    public static function to(string $path): static
    {}

    /**
     * Get a URI instance for a named route.
     *
     * @param  \BackedEnum|string  $name
     * @param  mixed  $parameters
     * @param  bool  $absolute
     * @return static
     *
     * @throws \Symfony\Component\Routing\Exception\RouteNotFoundException|\InvalidArgumentException
     */
    public static function route($name, $parameters = [], $absolute = true): static
    {}

    /**
     * Create a signed route URI instance for a named route.
     *
     * @param  \BackedEnum|string  $name
     * @param  mixed  $parameters
     * @param  \DateTimeInterface|\DateInterval|int|null  $expiration
     * @param  bool  $absolute
     * @return static
     *
     * @throws \InvalidArgumentException
     */
    public static function signedRoute($name, $parameters = [], $expiration = null, $absolute = true): static
    {}

    /**
     * Create a temporary signed route URI instance for a named route.
     *
     * @param  \BackedEnum|string  $name
     * @param  \DateTimeInterface|\DateInterval|int  $expiration
     * @param  array  $parameters
     * @param  bool  $absolute
     * @return static
     */
    public static function temporarySignedRoute($name, $expiration, $parameters = [], $absolute = true): static
    {}

    /**
     * Get a URI instance for a controller action.
     *
     * @param  string|array  $action
     * @param  mixed  $parameters
     * @param  bool  $absolute
     * @return static
     *
     * @throws \InvalidArgumentException
     */
    public static function action($action, $parameters = [], $absolute = true): static
    {}

    /**
     * Get the URI's authority.
     */
    public function authority(): ?string
    {}

    /**
     * Get the URI's scheme.
     */
    public function scheme(): ?string
    {}

    /**
     * Get the user from the URI.
     */
    public function user(bool $withPassword = false): ?string
    {}

    /**
     * Get the password from the URI.
     */
    public function password(): ?string
    {}

    /**
     * Get the URI's host.
     */
    public function host(): ?string
    {}

    /**
     * Get the URI's port.
     */
    public function port(): ?int
    {}

    /**
     * Get the URI's path.
     *
     * Empty or missing paths are returned as a single "/".
     *
     * @return non-empty-string
     */
    public function path(): string
    {}

    /**
     * Get the URI's path segments.
     *
     * Empty or missing paths are returned as an empty collection.
     */
    public function pathSegments(): Collection
    {}

    /**
     * Get the URI's query string.
     */
    public function query(): UriQueryString
    {}

    /**
     * Get the URI's fragment.
     */
    public function fragment(): ?string
    {}

    /**
     * Specify the scheme of the URI.
     */
    public function withScheme(Stringable|string $scheme): static
    {}

    /**
     * Specify the user and password for the URI.
     */
    public function withUser(Stringable|string|null $user, #[SensitiveParameter] Stringable|string|null $password = null): static
    {}

    /**
     * Specify the host of the URI.
     */
    public function withHost(Stringable|string $host): static
    {}

    /**
     * Specify the port of the URI.
     */
    public function withPort(?int $port): static
    {}

    /**
     * Specify the path of the URI.
     */
    public function withPath(Stringable|string $path): static
    {}

    /**
     * Merge new query parameters into the URI.
     */
    public function withQuery(array $query, bool $merge = true): static
    {}

    /**
     * Merge new query parameters into the URI if they are not already in the query string.
     */
    public function withQueryIfMissing(array $query): static
    {}

    /**
     * Push a value onto the end of a query string parameter that is a list.
     */
    public function pushOntoQuery(string $key, mixed $value): static
    {}

    /**
     * Remove the given query parameters from the URI.
     */
    public function withoutQuery(array|string $keys): static
    {}

    /**
     * Specify new query parameters for the URI.
     */
    public function replaceQuery(array $query): static
    {}

    /**
     * Specify the fragment of the URI.
     */
    public function withFragment(string $fragment): static
    {}

    /**
     * Remove the fragment from the URI.
     */
    public function withoutFragment(): static
    {}

    /**
     * Create a redirect HTTP response for the given URI.
     */
    public function redirect(int $status = 302, array $headers = []): RedirectResponse
    {}

    /**
     * Get the URI as a Stringable instance.
     *
     * @return \Illuminate\Support\Stringable
     */
    public function toStringable()
    {}

    /**
     * Create an HTTP response that represents the URI object.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function toResponse($request)
    {}

    /**
     * Get the URI as a string of HTML.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Get the decoded string representation of the URI.
     */
    public function decode(): string
    {}

    /**
     * Get the string representation of the URI.
     */
    public function value(): string
    {}

    /**
     * Get the string representation of the URI.
     */
    public function toString(): string
    {}

    /**
     * Determine if the URI is currently an empty string.
     */
    public function isEmpty(): bool
    {}

    /**
     * Determine if the URI is not an empty string.
     */
    public function isNotEmpty(): bool
    {}

    /**
     * Dump the string representation of the URI.
     *
     * @param  mixed  ...$args
     * @return $this
     */
    public function dump(...$args)
    {}

    /**
     * Set the URL generator resolver.
     */
    public static function setUrlGeneratorResolver(Closure $urlGeneratorResolver): void
    {}

    /**
     * Get the underlying URI instance.
     */
    public function getUri(): UriInterface
    {}

    /**
     * Convert the object into a value that is JSON serializable.
     *
     * @return string
     */
    public function jsonSerialize(): string
    {}

    /**
     * Get the string representation of the URI.
     */
    public function __toString(): string
    {}
}
