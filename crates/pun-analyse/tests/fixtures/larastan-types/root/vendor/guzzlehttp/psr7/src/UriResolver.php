<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\UriInterface;

/**
 * Resolves a URI reference in the context of a base URI and the opposite way.
 *
 * @author Tobias Schultze
 *
 * @see https://datatracker.ietf.org/doc/html/rfc3986#section-5
 */
final class UriResolver
{
    /**
     * Removes dot segments from a path and returns the new path according to
     * RFC 3986 Section 5.2.4.
     *
     * Excess `..` segments above the root of an absolute path are dropped
     * without consuming the root, so the result can begin with `//` (e.g.
     * `/..//a` becomes `//a`). Such a path is not valid for a URI without an
     * authority (RFC 3986 Section 3.3); `resolve()` and
     * `UriNormalizer::normalize()` serialize it with a `/.` prefix in that
     * case, like the WHATWG URL Standard.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-5.2.4
     */
    public static function removeDotSegments(string $path): string
    {}

    /**
     * Returns the path, prefixed with "/." or "./" when the URI could not
     * otherwise hold it.
     *
     * A URI without an authority cannot hold a path beginning with "//" (RFC
     * 3986 Section 3.3), but removeDotSegments() can produce one. The "/."
     * prefix serializes such a path unambiguously, the same way the WHATWG URL
     * Standard does, and resolves back to the same path. Hostless http and
     * https Uri instances gain the default localhost host when the path is
     * written, so the path cannot be mistaken for an authority and the prefix
     * is not added.
     *
     * A relative-path reference cannot begin with a segment containing a colon
     * (RFC 3986 Section 4.2), as it would be mistaken for a scheme name, but
     * reference resolution and percent-encoding normalization can produce one.
     * The "./" prefix the RFC prescribes resolves back to the same path.
     *
     * @see https://url.spec.whatwg.org/#url-serializing
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
     *
     * @internal
     */
    public static function guardedPath(UriInterface $uri, string $path): string
    {}

    /**
     * Converts the relative URI into a new URI that is resolved against the
     * base URI.
     *
     * When the resolved path is a relative-path reference whose first segment
     * contains a colon, which would be mistaken for a scheme name (RFC 3986
     * Section 4.2), it is prefixed with `./`, e.g. `./a:b`.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-5.2
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
     */
    public static function resolve(UriInterface $base, UriInterface $rel): UriInterface
    {}

    /**
     * Returns the target URI as a relative reference from the base URI.
     *
     * This method is the counterpart to `resolve()`:
     *
     *    (string) $target === (string) UriResolver::resolve($base, UriResolver::relativize($base, $target))
     *
     * One use case is to use the current request URI as the base URI and then
     * generate relative links in your documents to reduce the document size or
     * offer self-contained downloadable document archives.
     *
     *    $base = new Uri('http://example.com/a/b/');
     *    echo UriResolver::relativize($base, new Uri('http://example.com/a/b/c'));  // prints 'c'.
     *    echo UriResolver::relativize($base, new Uri('http://example.com/a/x/y'));  // prints '../x/y'.
     *    echo UriResolver::relativize($base, new Uri('http://example.com/a/b/?q')); // prints '?q'.
     *    echo UriResolver::relativize($base, new Uri('http://example.org/a/b/'));   // prints '//example.org/a/b/'.
     *    echo UriResolver::relativize($base, new Uri('http://example.com'));         // prints '//example.com'.
     *
     * This method also accepts a target that is already relative and will try
     * to relativize it further. Only a relative-path reference will be returned
     * as-is.
     *
     *    echo UriResolver::relativize($base, new Uri('/a/b/c'));  // prints 'c' as well
     */
    public static function relativize(UriInterface $base, UriInterface $target): UriInterface
    {}

    /**
     * Whether relativizing to $target requires a network-path reference.
     *
     * A same-authority target with an empty path is expressible by a shorter
     * relative reference unless resolving one would inherit a base component
     * the target lacks: the base path (kept by any empty-path reference), or
     * the base query or fragment (inherited by the empty reference).
     */
    private static function needsNetworkPathReference(UriInterface $base, UriInterface $target): bool
    {}

    private static function getRelativePath(UriInterface $base, UriInterface $target): string
    {}

    private function __construct()
    {}
}
