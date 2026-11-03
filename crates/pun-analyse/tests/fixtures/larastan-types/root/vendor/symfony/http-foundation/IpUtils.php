<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

/**
 * Http utility functions.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class IpUtils
{
    public const PRIVATE_SUBNETS = [
        '127.0.0.0/8',     // RFC1700 (Loopback)
        '10.0.0.0/8',      // RFC1918
        '192.168.0.0/16',  // RFC1918
        '192.0.2.0/24',    // Documentation Ranges TEST-NET-1 (RFC 5737)
        '198.51.100.0/24', // Documentation Ranges TEST-NET-2 (RFC 5737)
        '203.0.113.0/24',  // Documentation Ranges TEST-NET-3 (RFC 5737)
        '172.16.0.0/12',   // RFC1918
        '169.254.0.0/16',  // RFC3927
        '198.18.0.0/15',   // IPv4 Benchmarking (RFC 2544)
        '0.0.0.0/8',       // RFC5735
        '240.0.0.0/4',     // RFC1112
        '100.64.0.0/10',   // RFC6598
        '::1/128',         // Loopback
        'fc00::/7',        // Unique Local Address
        'fe80::/10',       // Link Local Address
        '::ffff:0:0/96',   // IPv4-mapped IPv6 addresses (RFC 4291 section 2.5.5.2)
        '::/128',          // Unspecified address
        '::/96',           // IPv4-compatible IPv6 addresses (RFC 4291 section 2.5.5.1)
        '2002::/16',       // 6to4 (RFC 3056)
        '2001::/32',       // Teredo tunneling (RFC 4380)
        '2001:db8::/32',   // Documentation Ranges (RFC 3849)
        '2001:0002::/48',  // IPv6 Benchmarking (RFC 5180 and corrections)
        '64:ff9b::/96',    // NAT64 well-known prefix (RFC 6052)
        '64:ff9b:1::/48',  // NAT64 local-use prefix (RFC 8215)
    ];

    private static array $checkedIps = [];

    /**
     * This class should not be instantiated.
     */
    private function __construct()
    {}

    /**
     * Checks if an IPv4 or IPv6 address is contained in the list of given IPs or subnets.
     *
     * @param string|array $ips List of IPs or subnets (can be a string if only a single one)
     */
    public static function checkIp(string $requestIp, string|array $ips): bool
    {}

    /**
     * Compares two IPv4 addresses.
     * In case a subnet is given, it checks if it contains the request IP.
     *
     * @param string $ip IPv4 address or subnet in CIDR notation
     *
     * @return bool Whether the request IP matches the IP, or whether the request IP is within the CIDR subnet
     */
    public static function checkIp4(string $requestIp, string $ip): bool
    {}

    /**
     * Compares two IPv6 addresses.
     * In case a subnet is given, it checks if it contains the request IP.
     *
     * @author David Soria Parra <dsp at php dot net>
     *
     * @see https://github.com/dsp/v6tools
     *
     * @param string $ip IPv6 address or subnet in CIDR notation
     *
     * @throws \RuntimeException When IPV6 support is not enabled
     */
    public static function checkIp6(string $requestIp, string $ip): bool
    {}

    /**
     * Anonymizes an IP/IPv6.
     *
     * Removes the last bytes of IPv4 and IPv6 addresses (1 byte for IPv4 and 8 bytes for IPv6 by default).
     *
     * @param int<0, 4>  $v4Bytes
     * @param int<0, 16> $v6Bytes
     */
    public static function anonymize(string $ip, int $v4Bytes = 1, int $v6Bytes = 8): string
    {}

    /**
     * Checks if an IPv4 or IPv6 address is contained in the list of private IP subnets.
     *
     * @throws \ValueError When $requestIp is not a valid IP address
     */
    public static function isPrivateIp(string $requestIp): bool
    {}

    private static function getCacheResult(string $cacheKey): ?bool
    {}

    private static function setCacheResult(string $cacheKey, bool $result): bool
    {}
}
