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

namespace League\Uri\IPv4;

use BackedEnum;
use League\Uri\Exceptions\MissingFeature;
use League\Uri\FeatureDetection;
use Stringable;

use function array_pop;
use function count;
use function explode;
use function extension_loaded;
use function hexdec;
use function long2ip;
use function ltrim;
use function preg_match;
use function str_ends_with;
use function substr;

use const FILTER_FLAG_IPV4;
use const FILTER_FLAG_IPV6;
use const FILTER_VALIDATE_IP;

final class Converter
{
    private const REGEXP_IPV4_HOST = '/
        (?(DEFINE) # . is missing as it is used to separate labels
            (?<hexadecimal>0x[[:xdigit:]]*)
            (?<octal>0[0-7]*)
            (?<decimal>\d+)
            (?<ipv4_part>(?:(?&hexadecimal)|(?&octal)|(?&decimal))*)
        )
        ^(?:(?&ipv4_part)\.){0,3}(?&ipv4_part)\.?$
    /x';
    private const REGEXP_IPV4_NUMBER_PER_BASE = [
        '/^0x(?<number>[[:xdigit:]]*)$/' => 16,
        '/^0(?<number>[0-7]*)$/' => 8,
        '/^(?<number>\d+)$/' => 10,
    ];

    private const IPV6_6TO4_PREFIX = '2002:';
    private const IPV4_MAPPED_PREFIX = '::ffff:';

    private readonly mixed $maxIPv4Number;

    public function __construct(
        private readonly Calculator $calculator
    ) {}

    /**
     * Returns an instance using a GMP calculator.
     */
    public static function fromGMP(): self
    {}

    /**
     * Returns an instance using a Bcmath calculator.
     */
    public static function fromBCMath(): self
    {}

    /**
     * Returns an instance using a PHP native calculator (requires 64bits PHP).
     */
    public static function fromNative(): self
    {}

    /**
     * Returns an instance using a detected calculator depending on the PHP environment.
     *
     * @throws MissingFeature If no Calculator implementing object can be used on the platform
     *
     * @codeCoverageIgnore
     */
    public static function fromEnvironment(): self
    {}

    public function isIpv4(BackedEnum|Stringable|string|null $host): bool
    {}

    public function toIPv6Using6to4(BackedEnum|Stringable|string|null $host): ?string
    {}

    public function toIPv6UsingMapping(BackedEnum|Stringable|string|null $host): ?string
    {}

    public function toOctal(BackedEnum|Stringable|string|null $host): ?string
    {}

    public function toHexadecimal(BackedEnum|Stringable|string|null $host): ?string
    {}

    /**
     * Tries to convert a IPv4 hexadecimal or a IPv4 octal notation into a IPv4 dot-decimal notation if possible
     * otherwise returns null.
     *
     * @see https://url.spec.whatwg.org/#concept-ipv4-parser
     */
    public function toDecimal(BackedEnum|Stringable|string|null $host): ?string
    {}

    /**
     * Converts a domain label into a IPv4 integer part.
     *
     * @see https://url.spec.whatwg.org/#ipv4-number-parser
     *
     * @return mixed returns null if it cannot correctly convert the label
     */
    private function labelToNumber(string $label): mixed
    {}

    /**
     * Generates the dot-decimal notation for IPv4.
     *
     * @see https://url.spec.whatwg.org/#concept-ipv4-parser
     *
     * @param mixed $ipAddress the number representation of the IPV4address
     */
    private function long2Ip(mixed $ipAddress): string
    {}
}
