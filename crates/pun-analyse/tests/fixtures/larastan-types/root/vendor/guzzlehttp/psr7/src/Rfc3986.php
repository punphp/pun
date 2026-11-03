<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

/**
 * Syntax predicates and canonicalization helpers for the URI grammar defined
 * by RFC 3986.
 */
final class Rfc3986
{
    private function __construct()
    {}

    /**
     * Sub-delims for use in a regex.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-2.2
     *
     * @internal
     */
    public const CHAR_SUB_DELIMS = '!\$&\'\(\)\*\+,;=';

    /**
     * Unreserved characters for use in a regex.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-2.3
     *
     * @internal
     */
    public const CHAR_UNRESERVED = 'a-zA-Z0-9_\-\.~';

    /**
     * The two hex digits of a percent-encoded octet (the "3A" in "%3A"), for use in a regex.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-2.1
     *
     * @internal
     */
    public const HEX_OCTET = '[A-Fa-f0-9]{2}';

    /**
     * Whether the string is a valid URI scheme.
     *
     * Per RFC 3986 a scheme must start with a letter, followed by letters,
     * digits, `+`, `-`, or `.`. The empty string is also accepted, since a URI
     * reference may omit the scheme.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
     */
    public static function isValidScheme(string $scheme): bool
    {}

    /**
     * Whether the string is a valid URI host.
     *
     * Per RFC 3986 the host is `IP-literal / IPv4address / reg-name`. An empty
     * host is accepted, since the authority (and thus the host) may be empty.
     * Bracketed values are validated as IPv6 / IPvFuture literals; any other
     * value is rejected if it contains control characters, whitespace, an
     * authority or path delimiter (`/ ? # @ \`), an embedded colon denoting
     * a port, a malformed percent-sequence, or a percent-encoded octet that
     * decodes to one of those rejected bytes, to a bracket, or to `%` itself.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2
     */
    public static function isValidHost(string $host): bool
    {}

    /**
     * Whether the string is a valid port number (0-65535).
     *
     * RFC 3986 defines the port as `*DIGIT`, which also permits an empty port
     * and has no upper bound. This applies the stricter policy used throughout
     * the library instead: the value must be a non-empty run of digits (leading
     * zeros are accepted and normalized) that resolves to 0-65535.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.3
     */
    public static function isValidPort(string $port): bool
    {}

    /**
     * Returns the RFC 5952 canonical form of a valid IPv6 address.
     *
     * The address must be a valid textual IPv6 address without brackets and
     * without a zone identifier, such as the inside of an IP-literal accepted
     * by `isValidHost()`. Canonicalization lowercases the hexadecimal fields,
     * suppresses leading zeros, and collapses the longest run of two or more
     * zero fields (the leftmost on a tie) with `::`. Embedded dotted-decimal
     * notation follows the rendering policy of BIND-derived `inet_ntop()`
     * implementations and curl: exactly the IPv4-mapped (`::ffff:0:0/96`) and
     * deprecated IPv4-compatible (`::/96`) layouts use it, while other
     * embedded-IPv4 forms, including translated (NAT64) well-known prefixes
     * such as `64:ff9b::/96` (RFC 6052), serialize in pure hexadecimal fields.
     *
     * Validation is strict and platform-independent: the address is checked
     * against the RFC 3986 `IPv6address` grammar with PHP's
     * `FILTER_VALIDATE_IP` filter and parsed in pure PHP, so spellings that
     * only some platform parsers accept, such as the zero-padded dotted octets
     * in `::ffff:192.168.001.001`, are rejected everywhere.
     *
     * @throws \InvalidArgumentException If the address cannot be parsed.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc5952#section-4
     */
    public static function canonicalizeIpv6(string $address): string
    {}

    /**
     * Returns the RFC 5952 canonical form of a valid IPv6 address, or null
     * when the address cannot be parsed.
     *
     * @internal
     */
    public static function tryCanonicalizeIpv6(string $address): ?string
    {}

    private static function hasValidHostPercentEncoding(string $host): bool
    {}

    private static function isValidIpLiteralHost(string $host): bool
    {}

    /**
     * Parses a textual IPv6 address into its eight 16-bit words, or null
     * when the text is not a structurally valid address.
     *
     * The grammar enforced here is the RFC 3986 `IPv6address` rule: one to four
     * hexadecimal digits per field, at most one `::` eliding one or more zero
     * fields, and an optional dotted-decimal tail of four octets (0-255, no
     * leading zeros) as the final 32 bits. FILTER_VALIDATE_IP accepts exactly
     * this grammar, so the filter guard in tryCanonicalizeIpv6() and this
     * parser always agree and the null paths here can only fail closed.
     *
     * @return list<int>|null
     */
    private static function parseIpv6Words(string $address): ?array
    {}

    /**
     * Parses a colon-separated run of 16-bit hexadecimal fields, or null
     * when a field is empty, longer than four digits, or not hexadecimal.
     *
     * @return list<int>|null
     */
    private static function parseHexFields(string $fields): ?array
    {}
}
