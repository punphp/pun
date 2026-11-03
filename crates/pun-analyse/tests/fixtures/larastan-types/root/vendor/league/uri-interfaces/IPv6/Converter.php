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

namespace League\Uri\IPv6;

use BackedEnum;
use Stringable;
use ValueError;

use function filter_var;
use function implode;
use function inet_pton;
use function str_split;
use function strtolower;
use function unpack;

use const FILTER_FLAG_IPV6;
use const FILTER_VALIDATE_IP;

final class Converter
{
    /**
     * Significant 10 bits of IP to detect Zone ID regular expression pattern.
     *
     * @var string
     */
    private const HOST_ADDRESS_BLOCK = "\xfe\x80";

    public static function compressIp(BackedEnum|string $ipAddress): string
    {}

    public static function expandIp(BackedEnum|string $ipAddress): string
    {}

    public static function compress(BackedEnum|Stringable|string|null $host): ?string
    {}

    public static function expand(Stringable|string|null $host): ?string
    {}

    public static function build(array $components): string
    {}

    /**
     * @return array{ipAddress:string|null, zoneIdentifier:string|null}
     */
    private static function parse(BackedEnum|Stringable|string|null $host): array
    {}

    /**
     * Tells whether the host is an IPv6.
     */
    public static function isIpv6(BackedEnum|Stringable|string|null $host): bool
    {}

    public static function normalize(BackedEnum|Stringable|string|null $host): ?string
    {}
}
