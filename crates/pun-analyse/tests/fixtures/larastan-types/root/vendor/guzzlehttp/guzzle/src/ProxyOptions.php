<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Psr7\Rfc3986;
use Psr\Http\Message\UriInterface;

final class ProxyOptions
{
    private function __construct()
    {}

    /**
     * Resolve Guzzle's documented proxy request option for a request URI.
     *
     * @param mixed $proxy Proxy option as passed via request transfer options.
     *
     * @throws InvalidArgumentException
     */
    public static function resolve(
        UriInterface $uri,
        #[\SensitiveParameter]
        $proxy
    ): ProxySelection {}

    /**
     * Validate a proxy URL and return its lowercased scheme.
     *
     * A proxy is an authority ([userinfo@]host[:port]) with an optional scheme;
     * a scheme-less value is an HTTP proxy. The scheme is matched anchored at
     * the start, so leading junk before it is rejected as malformed, and the
     * host and port grammar is delegated to Psr7\Rfc3986. The whole string is
     * validated up front so a malformed proxy fails the same way on every
     * handler, but the original value is what callers pin, so no normalization
     * reaches the wire. The error message never includes the proxy, which may
     * carry credentials.
     *
     * @throws InvalidArgumentException on a malformed proxy URL
     */
    public static function proxyScheme(
        #[\SensitiveParameter]
        string $proxy
    ): string {}

    /**
     * Whether the string is a valid proxy authority: [userinfo@]host[:port].
     *
     * A single trailing slash is tolerated; anything else after the authority
     * (a path, query, or fragment) is rejected. Userinfo is not policed here
     * (the proxy handles its own credentials); host and port grammar are
     * delegated to Psr7\Rfc3986, where a zero port is accepted and left for the
     * transport to handle as it did before.
     */
    private static function isValidProxyAuthority(string $authority): bool
    {}

    /**
     * Normalize a no-proxy list from request options or NO_PROXY.
     *
     * @param mixed $noProxy No-proxy value as passed via request transfer options.
     *
     * @return string[]
     *
     * @throws InvalidArgumentException
     */
    public static function normalizeNoProxy($noProxy): array
    {}

    /**
     * Returns true if the provided URI matches any of the no-proxy areas.
     *
     * @param string[] $noProxy An array of host, host-and-port, or CIDR patterns.
     *
     * @throws InvalidArgumentException
     */
    public static function isUriInNoProxy(UriInterface $uri, array $noProxy): bool
    {}

    /**
     * Returns true if the provided host matches any of the no-proxy areas.
     *
     * This method will strip a port from the host if it is present. Domain
     * patterns are matched case-insensitively. Exact IP literal patterns are
     * matched by their normalized binary address, and a host or pattern
     * written in the inet_aton() shorthand a transport reads as an address,
     * such as 127.1 or 0x7f000001, is matched as that address.
     *
     * Areas are matched in the following cases:
     * 1. "*" (without quotes) always matches any hosts.
     * 2. An exact domain or IP literal match.
     * 3. A bare domain or a leading-dot domain matches itself and its
     *    subdomains. e.g. 'mit.edu' and '.mit.edu' both match 'mit.edu'
     *    and 'foo.mit.edu'.
     * 4. IP CIDR entries match IP literal hosts. e.g. '192.168.0.0/16' will
     *    match '192.168.1.10' and 'fd00::/8' will match '[fd00::1]'.
     *
     * @param string   $host    Host to check against the patterns.
     * @param string[] $noProxy An array of host or CIDR patterns.
     *
     * @throws InvalidArgumentException
     */
    public static function isHostInNoProxy(string $host, array $noProxy): bool
    {}

    /**
     * @param array<array-key, mixed> $noProxy
     *
     * @throws InvalidArgumentException
     */
    private static function assertNoProxyList(array $noProxy): void
    {}

    /**
     * @return array{type: string, value: string, port: int|null}|null
     */
    private static function parseNoProxyTarget(UriInterface $uri): ?array
    {}

    /**
     * @return array{type: string, value: string, port: int|null}|null
     */
    private static function parseNoProxyHostString(string $host): ?array
    {}

    /**
     * @return array{type: string, value: string, port: int|null}|array{type: string, value: string, prefix: int}|null
     */
    private static function parseNoProxyRule(string $area): ?array
    {}

    /**
     * @return array{type: string, value: string, port: int|null}|null
     */
    private static function parseNoProxyHost(string $host, ?int $port): ?array
    {}

    /**
     * @return array{0: string, 1: int|null}|null
     */
    private static function splitNoProxyHostAndPort(string $area): ?array
    {}

    private static function parseNoProxyPort(string $port): ?int
    {}

    private static function getDefaultPort(string $scheme): ?int
    {}

    /**
     * @return array{type: string, value: string, prefix: int}|null
     */
    private static function parseNoProxyCidrRule(string $area): ?array
    {}

    private static function parseBoundedUnsignedInteger(string $value, int $max): ?int
    {}

    /**
     * @param array{type: string, value: string, port: int|null}                     $target
     * @param array{type: string, value: string, port?: int|null, prefix?: int|null} $rule
     */
    private static function noProxyRuleMatches(array $target, array $rule): bool
    {}

    /**
     * @return string|false
     */
    private static function packIpAddress(string $ip)
    {}

    /**
     * @return string|false
     */
    private static function packHostAddress(string $host)
    {}

    private static function ipMatchesPrefix(string $address, string $network, int $prefix): bool
    {}
}
