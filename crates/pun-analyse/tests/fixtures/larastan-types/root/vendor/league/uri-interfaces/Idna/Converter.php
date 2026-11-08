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

namespace League\Uri\Idna;

use BackedEnum;
use League\Uri\Exceptions\ConversionFailed;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\FeatureDetection;
use Stringable;

use function idn_to_ascii;
use function idn_to_utf8;
use function preg_match;
use function preg_replace_callback;
use function rawurldecode;
use function strtolower;
use function strtoupper;

use const INTL_IDNA_VARIANT_UTS46;

/**
 * @see https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/uidna_8h.html
 */
final class Converter
{
    private const REGEXP_IDNA_PATTERN = '/[^\x20-\x7f]/';
    private const MAX_DOMAIN_LENGTH = 253;
    private const MAX_LABEL_LENGTH = 63;
    private const REGEXP_CHARS_ENCODED = ',%[A-Fa-f0-9]{2},';
    private const REGEXP_PART_UNRESERVED = 'A-Za-z\d_\-.~';

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
     * Converts the input to its IDNA ASCII form or throw on failure.
     *
     * @see Converter::toAscii()
     *
     * @throws SyntaxError if the string cannot be converted to UNICODE using IDN UTS46 algorithm
     * @throws ConversionFailed if the conversion returns error
     */
    public static function toAsciiOrFail(BackedEnum|Stringable|string $domain, Option|int|null $options = null): string
    {}

    /**
     * Converts the input to its IDNA ASCII form.
     *
     * This method returns the string converted to IDN ASCII form
     *
     * @throws SyntaxError if the string cannot be converted to ASCII using IDN UTS46 algorithm
     */
    public static function toAscii(BackedEnum|Stringable|string $domain, Option|int|null $options = null): Result
    {}

    /**
     * Converts the input to its IDNA UNICODE form or throw on failure.
     *
     * @see Converter::toUnicode()
     *
     * @throws ConversionFailed if the conversion returns error
     */
    public static function toUnicodeOrFail(BackedEnum|Stringable|string $domain, Option|int|null $options = null): string
    {}

    /**
     * Converts the input to its IDNA UNICODE form.
     *
     * This method returns the string converted to IDN UNICODE form
     *
     * @throws SyntaxError if the string cannot be converted to UNICODE using IDN UTS46 algorithm
     */
    public static function toUnicode(BackedEnum|Stringable|string $domain, Option|int|null $options = null): Result
    {}

    private static function filterDomain(BackedEnum|Stringable|string $domain): string
    {}

    /**
     * Tells whether the submitted host is a valid IDN regardless of its format.
     *
     * Returns false if the host is invalid or if its conversion yields the same result
     */
    public static function isIdn(BackedEnum|Stringable|string|null $domain): bool
    {}

    /**
     * Adapted from https://github.com/TRowbotham/idna.
     *
     * @see https://github.com/TRowbotham/idna/blob/master/src/Idna.php#L236
     */
    private static function validateDomainAndLabelLength(string $domain): int
    {}
}
