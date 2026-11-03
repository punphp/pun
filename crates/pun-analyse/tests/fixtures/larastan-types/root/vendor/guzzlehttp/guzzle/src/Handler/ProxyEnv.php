<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\ProxyOptions;
use GuzzleHttp\ProxySelection;
use GuzzleHttp\Psr7;
use Psr\Http\Message\UriInterface;

/**
 * Resolves proxy configuration from the process environment with the same
 * semantics libcurl applies, shared by the built-in handlers: the cURL
 * handlers pin CURLOPT_PROXY and CURLOPT_NOPROXY explicitly so libcurl never
 * reads the environment itself, and the stream handler resolves the same way.
 *
 * @internal
 */
final class ProxyEnv
{
    private function __construct()
    {}

    /**
     * Resolves the proxy selection for a request, falling back to the proxy
     * environment variables when the proxy request option makes no decision.
     *
     * The environment no_proxy list is tokenized the way libcurl tokenizes
     * it and matched here with the same rules as the proxy option's "no"
     * list, so behavior does not depend on the installed libcurl's matcher.
     *
     * @param mixed $proxyOption
     */
    public static function resolveProxySelection(
        UriInterface $uri,
        #[\SensitiveParameter]
        $proxyOption
    ): ProxySelection {}

    /**
     * Resolves the proxy to use for the given request scheme.
     *
     * The lookup mirrors libcurl for the http and https schemes the handlers
     * accept: the lowercase scheme-specific variable first, its uppercase
     * variant next (except for "http", where uppercase HTTP_PROXY is never
     * read), then all_proxy/ALL_PROXY.
     *
     * @return string|null The proxy to use; null when the environment
     *                     configures none.
     */
    public static function getProxyForScheme(string $scheme): ?string
    {}

    /**
     * @return string|null The no-proxy list; null when nothing is set.
     */
    public static function getNoProxy(): ?string
    {}

    /**
     * Splits a no_proxy environment value into matchable entries.
     *
     * Entries may be separated by commas or blanks, the way libcurl tokenizes
     * the value.
     *
     * @return string[]
     */
    public static function splitNoProxy(string $noProxy): array
    {}

    private static function getenv(string $name): ?string
    {}
}
