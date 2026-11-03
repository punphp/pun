<?php

declare(strict_types=1);

namespace GuzzleHttp;

/**
 * Shared host identity normalization for features that key state on a
 * logical host.
 *
 * @internal
 */
final class HostIdentity
{
    /**
     * Returns the identity form of a URI host. Valid bracketed IPv6 literals
     * are canonicalized to their RFC 5952 form so equivalent spellings of one
     * address share a single identity; IPvFuture literals, zone-bearing
     * values, and invalid bracketed text fall back to ASCII case folding.
     */
    public static function canonicalHost(string $host): string
    {}

    /**
     * Returns the identity form of a cookie domain. In addition to the
     * canonicalHost() rules, valid bare IPv6 addresses, which the cookie API
     * permissively accepts even though a URI host requires brackets, are
     * canonicalized to their RFC 5952 form without gaining brackets, so bare
     * and bracketed forms remain distinct identities.
     */
    public static function canonicalCookieDomain(string $domain): string
    {}

    /**
     * Whether a request domain matches a cookie domain according to RFC 6265.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc6265#section-5.1.3
     */
    public static function cookieDomainMatches(string $domain, string $cookieDomain): bool
    {}

    /**
     * Returns the identity form of a Host header value. A single bracketed
     * `host[:port]` authority, where any explicit port must be a valid decimal
     * port number, has its IPv6 literal canonicalized like canonicalHost() with
     * the port text retained; every other value, including malformed and
     * multiple values, falls back to ASCII case folding of the raw text.
     */
    public static function canonicalHostHeader(string $header): string
    {}

    /**
     * Whether the host may participate in RFC 6265 DNS suffix matching. IP
     * literals, IP addresses, numeric hosts, literal-like text containing a
     * raw bracket or colon, and invalid host text are exact-match-only.
     */
    private static function isDnsSuffixEligible(string $host): bool
    {}

    private static function isIpAddressOrNumericHost(string $host): bool
    {}

    /**
     * Returns the four-byte binary form of a host that a transport reads as a
     * numeric IPv4 address, or null when it reads it as a name.
     *
     * The shape test is Handler\HostValidator::isNumericIpv4Host(); this
     * method adds the range checks that predicate omits: every part but the
     * last must fit one octet, and the last must fit the octets the earlier
     * parts left. A trailing root dot is not swallowed, unlike libcurl 8.21.0
     * and later, because assertRequestHost() rejects that spelling first.
     */
    public static function numericIpv4ToBinary(string $host): ?string
    {}

    /**
     * Returns the value of one accepted part as a float, so a part filling
     * all four octets such as 2130706433 stays exact on every integer width.
     */
    private static function numericIpv4PartValue(string $part): float
    {}

    private static function digitsValue(string $digits, int $base): float
    {}

    private function __construct()
    {}
}
