<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com> and Trevor Rowbotham <trevor.rowbotham@pm.me>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Intl\Idn;

use Symfony\Polyfill\Intl\Idn\Resources\unidata\DisallowedRanges;
use Symfony\Polyfill\Intl\Idn\Resources\unidata\Regex;

/**
 * @see https://www.unicode.org/reports/tr46/
 *
 * @internal
 */
final class Idn
{
    public const ERROR_EMPTY_LABEL = 1;
    public const ERROR_LABEL_TOO_LONG = 2;
    public const ERROR_DOMAIN_NAME_TOO_LONG = 4;
    public const ERROR_LEADING_HYPHEN = 8;
    public const ERROR_TRAILING_HYPHEN = 0x10;
    public const ERROR_HYPHEN_3_4 = 0x20;
    public const ERROR_LEADING_COMBINING_MARK = 0x40;
    public const ERROR_DISALLOWED = 0x80;
    public const ERROR_PUNYCODE = 0x100;
    public const ERROR_LABEL_HAS_DOT = 0x200;
    public const ERROR_INVALID_ACE_LABEL = 0x400;
    public const ERROR_BIDI = 0x800;
    public const ERROR_CONTEXTJ = 0x1000;
    public const ERROR_CONTEXTO_PUNCTUATION = 0x2000;
    public const ERROR_CONTEXTO_DIGITS = 0x4000;

    public const INTL_IDNA_VARIANT_2003 = 0;
    public const INTL_IDNA_VARIANT_UTS46 = 1;

    public const IDNA_DEFAULT = 0;
    public const IDNA_ALLOW_UNASSIGNED = 1;
    public const IDNA_USE_STD3_RULES = 2;
    public const IDNA_CHECK_BIDI = 4;
    public const IDNA_CHECK_CONTEXTJ = 8;
    public const IDNA_NONTRANSITIONAL_TO_ASCII = 16;
    public const IDNA_NONTRANSITIONAL_TO_UNICODE = 32;

    public const MAX_DOMAIN_SIZE = 253;
    public const MAX_LABEL_SIZE = 63;

    public const BASE = 36;
    public const TMIN = 1;
    public const TMAX = 26;
    public const SKEW = 38;
    public const DAMP = 700;
    public const INITIAL_BIAS = 72;
    public const INITIAL_N = 128;
    public const DELIMITER = '-';
    public const MAX_INT = 2147483647;

    /**
     * Punycode decoding does work quadratic in the payload length. Valid ACE
     * labels are limited to 63 bytes, so payloads beyond this (generous) bound
     * are rejected without being decoded to keep the work bounded.
     */
    private const MAX_DECODE_PAYLOAD_SIZE = 1024;

    /**
     * Encoding is quadratic the same way: it is O(n*r) in the length of a label
     * and in its number of distinct code points. ext-intl never runs it on a
     * domain this large, because it fails before converting when the result
     * would not fit its own output buffer.
     */
    private const MAX_CODE_POINTS = 255;

    /**
     * Contains the numeric value of a basic code point (for use in representing integers) in the
     * range 0 to BASE-1, or -1 if b is does not represent a value.
     *
     * @var array<int, int>
     */
    private static $basicToDigit = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,

        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, -1, -1,

        -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,

        -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,

        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,

        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,

        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,

        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ];

    /**
     * @var array<int, int>
     */
    private static $virama;

    /**
     * @var array<int, string>
     */
    private static $mapped;

    /**
     * @var array<int, bool>
     */
    private static $ignored;

    /**
     * @var array<int, string>
     */
    private static $deviation;

    /**
     * @var array<int, bool>
     */
    private static $disallowed;

    /**
     * @var array<int, string>
     */
    private static $disallowed_STD3_mapped;

    /**
     * @var array<int, bool>
     */
    private static $disallowed_STD3_valid;

    /**
     * @var bool
     */
    private static $mappingTableLoaded = false;

    /**
     * @see https://www.unicode.org/reports/tr46/#ToASCII
     *
     * @param string $domainName
     * @param int    $options
     * @param int    $variant
     * @param array  $idna_info
     *
     * @return string|false
     */
    public static function idn_to_ascii($domainName, $options = self::IDNA_DEFAULT, $variant = self::INTL_IDNA_VARIANT_UTS46, &$idna_info = [])
    {}

    /**
     * @see https://www.unicode.org/reports/tr46/#ToUnicode
     *
     * @param string $domainName
     * @param int    $options
     * @param int    $variant
     * @param array  $idna_info
     *
     * @return string|false
     */
    public static function idn_to_utf8($domainName, $options = self::IDNA_DEFAULT, $variant = self::INTL_IDNA_VARIANT_UTS46, &$idna_info = [])
    {}

    /**
     * @param string $label
     *
     * @return bool
     */
    private static function isValidContextJ(array $codePoints, $label)
    {}

    /**
     * @see https://www.unicode.org/reports/tr46/#ProcessingStepMap
     *
     * @param string              $input
     * @param array<string, bool> $options
     *
     * @return string
     */
    private static function mapCodePoints($input, array $options, Info $info)
    {}

    /**
     * @see https://www.unicode.org/reports/tr46/#Processing
     *
     * @param string              $domain
     * @param array<string, bool> $options
     *
     * @return array<int, string>
     */
    private static function process($domain, array $options, Info $info)
    {}

    /**
     * @see https://tools.ietf.org/html/rfc5893#section-2
     *
     * @param string $label
     */
    private static function validateBidiLabel($label, Info $info)
    {}

    /**
     * @param array<int, string> $labels
     */
    private static function validateDomainAndLabelLength(array $labels, Info $info)
    {}

    /**
     * @see https://www.unicode.org/reports/tr46/#Validity_Criteria
     *
     * @param string              $label
     * @param array<string, bool> $options
     * @param bool                $canBeEmpty
     */
    private static function validateLabel($label, Info $info, array $options, $canBeEmpty)
    {}

    /**
     * @see https://tools.ietf.org/html/rfc3492#section-6.2
     *
     * @param string $input
     *
     * @return string
     */
    private static function punycodeDecode($input)
    {}

    /**
     * @see https://tools.ietf.org/html/rfc3492#section-6.3
     *
     * @param string $input
     *
     * @return string
     */
    private static function punycodeEncode($input)
    {}

    /**
     * Counts the code points of a UTF-8 string, malformed bytes included.
     *
     * This is the expression Mbstring::mb_strlen() falls back to in
     * symfony/polyfill-mbstring, inlined because this package does not depend on it.
     * Counting only the bytes that can start a sequence would be shorter, but then a
     * run of continuation bytes would pad a label without being counted, which is the
     * one thing this bound has to prevent.
     *
     * @param string $input
     *
     * @return int
     */
    private static function countCodePoints($input)
    {}

    /**
     * @see https://tools.ietf.org/html/rfc3492#section-6.1
     *
     * @param int  $delta
     * @param int  $numPoints
     * @param bool $firstTime
     *
     * @return int
     */
    private static function adaptBias($delta, $numPoints, $firstTime)
    {}

    /**
     * @param int  $d
     * @param bool $flag
     *
     * @return string
     */
    private static function encodeDigit($d, $flag)
    {}

    /**
     * Takes a UTF-8 encoded string and converts it into a series of integer code points. Any
     * invalid byte sequences will be replaced by a U+FFFD replacement code point.
     *
     * @see https://encoding.spec.whatwg.org/#utf-8-decoder
     *
     * @param string $input
     *
     * @return array<int, int>
     */
    private static function utf8Decode($input)
    {}

    /**
     * @param int  $codePoint
     * @param bool $useSTD3ASCIIRules
     *
     * @return array{status: string, mapping?: string}
     */
    private static function lookupCodePointStatus($codePoint, $useSTD3ASCIIRules)
    {}
}
