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
use Closure;
use Deprecated;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\IPv6\Converter as IPv6Converter;
use SensitiveParameter;
use Stringable;
use Throwable;

use function explode;
use function filter_var;
use function gettype;
use function preg_match;
use function preg_replace_callback;
use function rawurldecode;
use function rawurlencode;
use function sprintf;
use function str_starts_with;
use function strtolower;
use function strtoupper;

use const FILTER_FLAG_IPV4;
use const FILTER_VALIDATE_IP;

final class Encoder
{
    private const REGEXP_CHARS_INVALID = '/[\x00-\x1f\x7f]/';
    private const REGEXP_CHARS_ENCODED = ',%[A-Fa-f0-9]{2},';
    private const REGEXP_CHARS_PREVENTS_DECODING = ',%2[A-F1-24-9]|3[0-9BD]|4[1-9A-F]|5[0-9AF]|6[1-9A-F]|7[0-9E],i';
    private const REGEXP_PART_SUBDELIM = "\!\$&'\(\)\*\+,;\=%";
    private const REGEXP_PART_UNRESERVED = 'A-Za-z\d_\-.~';
    private const REGEXP_PART_ENCODED = '%(?![A-Fa-f\d]{2})';
    private const RFC3986_RESERVED_CHARACTERS = ':/?#[]@!$&\'()*+,;=';

    /**
     * Unreserved characters.
     *
     * @see https://www.rfc-editor.org/rfc/rfc3986.html#section-2.3
     */
    private const REGEXP_UNRESERVED_CHARACTERS = ',%(2[DdEe]|3[0-9]|4[1-9A-Fa-f]|5[0-9AaFf]|6[1-9A-Fa-f]|7[0-9AaEe]),';

    /**
     * Tell whether the user component is correctly encoded.
     */
    public static function isUserEncoded(BackedEnum|Stringable|string|null $encoded): bool
    {}

    /**
     * Encode User.
     *
     * All generic delimiters MUST be encoded
     */
    public static function encodeUser(BackedEnum|Stringable|string|null $user): ?string
    {}

    /**
     * Normalize user component.
     *
     * The value returned MUST be percent-encoded, but MUST NOT double-encode
     * any characters. To determine what characters to encode, please refer to
     * RFC 3986.
     */
    public static function normalizeUser(BackedEnum|Stringable|string|null $user): ?string
    {}

    private static function normalize(?string $component): ?string
    {}

    /**
     * Tell whether the password component is correctly encoded.
     */
    public static function isPasswordEncoded(#[SensitiveParameter] BackedEnum|Stringable|string|null $encoded): bool
    {}

    /**
     * Encode Password.
     *
     * Generic delimiters ":" MUST NOT be encoded
     */
    public static function encodePassword(#[SensitiveParameter] BackedEnum|Stringable|string|null $component): ?string
    {}

    /**
     * Normalize password component.
     *
     * The value returned MUST be percent-encoded, but MUST NOT double-encode
     * any characters. To determine what characters to encode, please refer to
     * RFC 3986.
     */
    public static function normalizePassword(#[SensitiveParameter] BackedEnum|Stringable|string|null $password): ?string
    {}

    /**
     * Tell whether the userInfo component is correctly encoded.
     */
    public static function isUserInfoEncoded(#[SensitiveParameter] BackedEnum|Stringable|string|null $userInfo): bool
    {}

    public static function encodeUserInfo(#[SensitiveParameter] BackedEnum|Stringable|string|null $userInfo): ?string
    {}

    public static function normalizeUserInfo(#[SensitiveParameter] BackedEnum|Stringable|string|null $userInfo): ?string
    {}

    /**
     * Decodes all the URI component characters.
     */
    public static function decodeAll(BackedEnum|Stringable|string|null $component): ?string
    {}

    /**
     * Decodes the URI component while preserving percent-encoded characters that must remain encoded.
     */
    public static function decodeNecessary(BackedEnum|Stringable|string|int|null $component): ?string
    {}

    /**
     * Decodes the component unreserved characters.
     */
    public static function decodeUnreservedCharacters(BackedEnum|Stringable|string|null $str): ?string
    {}

    /**
     * Tell whether the path component is correctly encoded.
     */
    public static function isPathEncoded(BackedEnum|Stringable|string|null $encoded): bool
    {}

    /**
     * Encode Path.
     *
     * Generic delimiters ":", "@", and "/" MUST NOT be encoded
     */
    public static function encodePath(BackedEnum|Stringable|string|null $component): string
    {}

    /**
     * Decodes the path component while preserving characters that should not be decoded in the context of a full valid URI.
     */
    public static function decodePath(BackedEnum|Stringable|string|null $path): ?string
    {}

    /**
     * Normalize path component.
     *
     * The value returned MUST be percent-encoded, but MUST NOT double-encode
     * any characters. To determine what characters to encode, please refer to
     * RFC 3986.
     */
    public static function normalizePath(BackedEnum|Stringable|string|null $component): ?string
    {}

    /**
     * Tell whether the query component is correctly encoded.
     */
    public static function isQueryEncoded(BackedEnum|Stringable|string|null $encoded): bool
    {}

    /**
     * Decodes the query component while preserving characters that should not be decoded in the context of a full valid URI.
     */
    public static function decodeQuery(BackedEnum|Stringable|string|null $query): ?string
    {}

    /**
     * Normalize the query component.
     *
     * The value returned MUST be percent-encoded, but MUST NOT double-encode
     * any characters. To determine what characters to encode, please refer to
     * RFC 3986.
     */
    public static function normalizeQuery(BackedEnum|Stringable|string|null $query): ?string
    {}

    /**
     * Tell whether the query component is correctly encoded.
     */
    public static function isFragmentEncoded(BackedEnum|Stringable|string|null $encoded): bool
    {}

    /**
     * Decodes the fragment component while preserving characters that should not be decoded in the context of a full valid URI.
     */
    public static function decodeFragment(BackedEnum|Stringable|string|null $fragment): ?string
    {}

    /**
     * Normalize the fragment component.
     *
     * The value returned MUST be percent-encoded, but MUST NOT double-encode
     * any characters. To determine what characters to encode, please refer to
     * RFC 3986.
     */
    public static function normalizeFragment(BackedEnum|Stringable|string|null $fragment): ?string
    {}

    /**
     * Normalize the host component.
     *
     * @see https://www.rfc-editor.org/rfc/rfc3986.html#section-3.2.2
     *
     * The value returned MUST be percent-encoded, but MUST NOT double-encode
     * any characters. To determine what characters to encode, please refer to
     * RFC 3986.
     */
    public static function normalizeHost(BackedEnum|Stringable|string|null $host): ?string
    {}

    /**
     * Encode Query or Fragment.
     *
     * Generic delimiters ":", "@", "?", and "/" MUST NOT be encoded
     */
    public static function encodeQueryOrFragment(BackedEnum|Stringable|string|null $component): ?string
    {}

    public static function encodeQueryKeyValue(mixed $component): string
    {}

    private static function filterComponent(mixed $component): ?string
    {}

    /**
     * Encodes the URI component characters using a regular expression to find which characters need encoding.
     */
    private static function encode(BackedEnum|Stringable|string|int|bool|null $component, string $pattern): ?string
    {}

    /**
     * Decodes the URI component characters using a closure.
     */
    private static function decode(BackedEnum|Stringable|string|int|null $component, Closure $decoder): ?string
    {}

    /**
     * Decodes the URI component without decoding the unreserved characters which are already encoded.
     *
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.6.0
     * @codeCoverageIgnore
     * @see Encoder::decodeNecessary()
     *
     * Create a new instance from the environment.
     */
    #[Deprecated(message:'use League\Uri\Encoder::decodeNecessary() instead', since:'league/uri:7.6.0')]
    public static function decodePartial(BackedEnum|Stringable|string|int|null $component): ?string
    {}
}
