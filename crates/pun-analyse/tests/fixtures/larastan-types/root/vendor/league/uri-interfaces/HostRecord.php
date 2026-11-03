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

use BackedEnum;
use Exception;
use JsonSerializable;
use League\Uri\Contracts\UriComponentInterface;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\Idna\Converter as IdnConverter;
use Stringable;
use Throwable;

use function array_key_first;
use function count;
use function explode;
use function filter_var;
use function get_object_vars;
use function in_array;
use function inet_pton;
use function is_object;
use function preg_match;
use function rawurldecode;
use function strpos;
use function strtolower;
use function substr;

use const FILTER_FLAG_IPV4;
use const FILTER_FLAG_IPV6;
use const FILTER_VALIDATE_IP;

/**
 * @phpstan-type HostRecordSerializedShape array{0: array{host: ?string}, 1: array{}}
 */
final class HostRecord implements JsonSerializable
{
    /**
     * Maximum number of host cached.
     *
     * @var int
     */
    private const MAXIMUM_HOST_CACHED = 100;

    private const REGEXP_NON_ASCII_PATTERN = '/[^\x20-\x7f]/';

    /**
     * @see https://tools.ietf.org/html/rfc3986#section-3.2.2
     *
     * invalid characters in host regular expression
     */
    private const REGEXP_INVALID_HOST_CHARS = '/
        [:\/?#\[\]@ ]  # gen-delims characters as well as the space character
    /ix';

    /**
     * General registered name regular expression.
     *
     * @see https://tools.ietf.org/html/rfc3986#section-3.2.2
     * @see https://regex101.com/r/fptU8V/1
     */
    private const REGEXP_REGISTERED_NAME = '/
    (?(DEFINE)
        (?<unreserved>[a-z0-9_~\-])   # . is missing as it is used to separate labels
        (?<sub_delims>[!$&\'()*+,;=])
        (?<encoded>%[A-F0-9]{2})
        (?<reg_name>(?:(?&unreserved)|(?&sub_delims)|(?&encoded))*)
    )
        ^(?:(?&reg_name)\.)*(?&reg_name)\.?$
    /ix';

    /**
     * Domain name regular expression.
     *
     * Everything but the domain name length is validated
     *
     * @see https://tools.ietf.org/html/rfc1034#section-3.5
     * @see https://tools.ietf.org/html/rfc1123#section-2.1
     * @see https://regex101.com/r/71j6rt/1
     */
    private const REGEXP_DOMAIN_NAME = '/
    (?(DEFINE)
        (?<let_dig> [a-z0-9])                         # alpha digit
        (?<let_dig_hyp> [a-z0-9-])                    # alpha digit and hyphen
        (?<ldh_str> (?&let_dig_hyp){0,61}(?&let_dig)) # domain label end
        (?<label> (?&let_dig)((?&ldh_str))?)          # domain label
        (?<domain> (?&label)(\.(?&label)){0,126}\.?)  # domain name
    )
        ^(?&domain)$
    /ix';

    /**
     * @see https://tools.ietf.org/html/rfc3986#section-3.2.2
     *
     * IPvFuture regular expression
     */
    private const REGEXP_IP_FUTURE = '/^
        v(?<version>[A-F\d])+\.
        (?:
            (?<unreserved>[a-z\d_~\-\.])|
            (?<sub_delims>[!$&\'()*+,;=:])  # also include the : character
        )+
    $/ix';
    private const REGEXP_GEN_DELIMS = '/[:\/?#\[\]@ ]/';
    private const ADDRESS_BLOCK = "\xfe\x80";

    private ?bool $isDomainName = null;
    private ?bool $hasZoneIdentifier = null;
    private bool $asciiIsLoaded = false;
    private ?string $hostAsAscii = null;
    private bool $unicodeIsLoaded = false;
    private ?string $hostAsUnicode = null;
    private bool $isIpVersionLoaded = false;
    private ?string $ipVersion = null;
    private bool $isIpValueLoaded = false;
    private ?string $ipValue = null;

    private function __construct(
        public readonly ?string $value,
        public readonly HostType $type,
        public readonly HostFormat $format
    ) {}

    public function hasZoneIdentifier(): bool
    {}

    public function toAscii(): ?string
    {}

    public function toUnicode(): ?string
    {}

    public function isDomainType(): bool
    {}

    public function ipVersion(): ?string
    {}

    public function ipValue(): ?string
    {}

    public static function isValid(BackedEnum|Stringable|string|null $host): bool
    {}

    public static function isIpv4(Stringable|string|null $host): bool
    {}

    public static function isIpv6(Stringable|string|null $host): bool
    {}

    public static function isIpvFuture(Stringable|string|null $host): bool
    {}

    public static function isIp(Stringable|string|null $host): bool
    {}

    public static function isRegisteredName(Stringable|string|null $host): bool
    {}

    public static function isDomain(Stringable|string|null $host): bool
    {}

    public static function parse(BackedEnum|Stringable|string|null $host): ?self
    {}

    /**
     * @throws SyntaxError
     */
    public static function from(BackedEnum|Stringable|string|null $host): self
    {}

    /**
     * Tells whether the registered name is a valid domain name according to RFC1123.
     *
     * @see http://man7.org/linux/man-pages/man7/hostname.7.html
     * @see https://tools.ietf.org/html/rfc1123#section-2.1
     */
    private static function isValidDomain(string $hostname): bool
    {}

    /**
     * Validates an Ipv6 as Host.
     *
     * @see http://tools.ietf.org/html/rfc6874#section-2
     * @see http://tools.ietf.org/html/rfc6874#section-4
     */
    private static function isValidIpv6Hostname(string $host): bool
    {}

    public function jsonSerialize(): ?string
    {}

    /**
     * @return HostRecordSerializedShape
     */
    public function __serialize(): array
    {}

    /**
     * @param HostRecordSerializedShape $data
     *
     * @throws Exception|SyntaxError
     */
    public function __unserialize(array $data): void
    {}
}
