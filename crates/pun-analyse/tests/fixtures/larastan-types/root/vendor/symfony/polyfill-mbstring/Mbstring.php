<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Mbstring;

/**
 * Partial mbstring implementation in PHP, iconv based, UTF-8 centric.
 *
 * Implemented:
 * - mb_chr                  - Returns a specific character from its Unicode code point
 * - mb_convert_encoding     - Convert character encoding
 * - mb_convert_variables    - Convert character code in variable(s)
 * - mb_decode_mimeheader    - Decode string in MIME header field
 * - mb_encode_mimeheader    - Encode string for MIME header XXX NATIVE IMPLEMENTATION IS REALLY BUGGED
 * - mb_decode_numericentity - Decode HTML numeric string reference to character
 * - mb_encode_numericentity - Encode character to HTML numeric string reference
 * - mb_convert_case         - Perform case folding on a string
 * - mb_detect_encoding      - Detect character encoding
 * - mb_get_info             - Get internal settings of mbstring
 * - mb_http_input           - Detect HTTP input character encoding
 * - mb_http_output          - Set/Get HTTP output character encoding
 * - mb_internal_encoding    - Set/Get internal character encoding
 * - mb_list_encodings       - Returns an array of all supported encodings
 * - mb_ord                  - Returns the Unicode code point of a character
 * - mb_output_handler       - Callback function converts character encoding in output buffer
 * - mb_scrub                - Replaces ill-formed byte sequences with substitute characters
 * - mb_strlen               - Get string length
 * - mb_strpos               - Find position of first occurrence of string in a string
 * - mb_strrpos              - Find position of last occurrence of a string in a string
 * - mb_str_split            - Convert a string to an array
 * - mb_strtolower           - Make a string lowercase
 * - mb_strtoupper           - Make a string uppercase
 * - mb_substitute_character - Set/Get substitution character
 * - mb_substr               - Get part of string
 * - mb_stripos              - Finds position of first occurrence of a string within another, case insensitive
 * - mb_stristr              - Finds first occurrence of a string within another, case insensitive
 * - mb_strrchr              - Finds the last occurrence of a character in a string within another
 * - mb_strrichr             - Finds the last occurrence of a character in a string within another, case insensitive
 * - mb_strripos             - Finds position of last occurrence of a string within another, case insensitive
 * - mb_strstr               - Finds first occurrence of a string within another
 * - mb_strwidth             - Return width of string
 * - mb_substr_count         - Count the number of substring occurrences
 * - mb_ucfirst              - Make a string's first character uppercase
 * - mb_lcfirst              - Make a string's first character lowercase
 * - mb_trim                 - Strip whitespace (or other characters) from the beginning and end of a string
 * - mb_ltrim                - Strip whitespace (or other characters) from the beginning of a string
 * - mb_rtrim                - Strip whitespace (or other characters) from the end of a string
 *
 * Not implemented:
 * - mb_convert_kana         - Convert "kana" one from another ("zen-kaku", "han-kaku" and more)
 * - mb_ereg_*               - Regular expression with multibyte support
 * - mb_parse_str            - Parse GET/POST/COOKIE data and set global variable
 * - mb_preferred_mime_name  - Get MIME charset string
 * - mb_regex_encoding       - Returns current encoding for multibyte regex as string
 * - mb_regex_set_options    - Set/Get the default options for mbregex functions
 * - mb_send_mail            - Send encoded mail
 * - mb_split                - Split multibyte string using regular expression
 * - mb_strcut               - Get part of string
 * - mb_strimwidth           - Get truncated string with specified width
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @internal
 */
final class Mbstring
{
    public const MB_CASE_FOLD = \PHP_INT_MAX;

    private const SIMPLE_CASE_FOLD = [
        ['µ', 'ſ', "\xCD\x85", 'ς', "\xCF\x90", "\xCF\x91", "\xCF\x95", "\xCF\x96", "\xCF\xB0", "\xCF\xB1", "\xCF\xB5", "\xE1\xBA\x9B", "\xE1\xBE\xBE"],
        ['μ', 's', 'ι',        'σ', 'β',        'θ',        'φ',        'π',        'κ',        'ρ',        'ε',        "\xE1\xB9\xA1", 'ι'],
    ];

    // Encodings rejected by mb_ord() and mb_chr(), with their aliases
    private const UNSUPPORTED_CODEPOINT_ENCODINGS = [
        'BASE64' => 'BASE64',
        'UUENCODE' => 'UUENCODE',
        'X-UUENCODE' => 'UUENCODE',
        'HTML-ENTITIES' => 'HTML-ENTITIES',
        'HTML' => 'HTML-ENTITIES',
        'QUOTED-PRINTABLE' => 'Quoted-Printable',
        'QPRINT' => 'Quoted-Printable',
        'UTF-7' => 'UTF-7',
        'UTF7' => 'UTF-7',
        'UTF7-IMAP' => 'UTF7-IMAP',
        'MUTF-7' => 'UTF7-IMAP',
        'JIS' => 'JIS',
        'ISO-2022-JP' => 'ISO-2022-JP',
        'ISO-2022-JP-2004' => 'ISO-2022-JP-2004',
        'ISO-2022-JP-MOBILE#KDDI' => 'ISO-2022-JP-MOBILE#KDDI',
        'ISO-2022-JP-KDDI' => 'ISO-2022-JP-MOBILE#KDDI',
        'ISO-2022-JP-MS' => 'ISO-2022-JP-MS',
        'ISO2022JPMS' => 'ISO-2022-JP-MS',
        'CP50220' => 'CP50220',
        'CP50220RAW' => 'CP50220',
        'CP50220-RAW' => 'CP50220',
        'JIS-MS' => 'CP50220',
        'CP50221' => 'CP50221',
        'CP50222' => 'CP50222',
    ];

    // Well-formed characters in group 1, then the maximal subparts of ill-formed sequences
    private const UTF8_CHAR_REGEX = '/([\x00-\x7F]|[\xC2-\xDF][\x80-\xBF]|\xE0[\xA0-\xBF][\x80-\xBF]|[\xE1-\xEC\xEE\xEF][\x80-\xBF]{2}|\xED[\x80-\x9F][\x80-\xBF]|\xF0[\x90-\xBF][\x80-\xBF]{2}|[\xF1-\xF3][\x80-\xBF]{3}|\xF4[\x80-\x8F][\x80-\xBF]{2})|\xE0[\xA0-\xBF]|[\xE1-\xEC\xEE\xEF][\x80-\xBF]|\xED[\x80-\x9F]|\xF0[\x90-\xBF][\x80-\xBF]?|[\xF1-\xF3][\x80-\xBF]{1,2}|\xF4[\x80-\x8F][\x80-\xBF]?|[\x80-\xFF]/';

    private static $encodingList = ['ASCII', 'UTF-8'];
    private static $language = 'neutral';
    private static $internalEncoding = 'UTF-8';
    private static $iconvSupportsIgnore;
    private static $convertibleEncodings = ['UTF-8' => true];

    public static function mb_convert_encoding($s, $toEncoding, $fromEncoding = null)
    {}

    public static function mb_convert_variables($toEncoding, $fromEncoding, &...$vars)
    {}

    public static function mb_decode_mimeheader($s)
    {}

    public static function mb_encode_mimeheader($s, $charset = null, $transferEncoding = null, $linefeed = null, $indent = null)
    {}

    public static function mb_decode_numericentity($s, $convmap, $encoding = null)
    {}

    public static function mb_encode_numericentity($s, $convmap, $encoding = null, $is_hex = false)
    {}

    public static function mb_convert_case($s, $mode, $encoding = null)
    {}

    public static function mb_internal_encoding($encoding = null)
    {}

    public static function mb_language($lang = null)
    {}

    public static function mb_list_encodings()
    {}

    public static function mb_encoding_aliases($encoding)
    {}

    public static function mb_check_encoding($var = null, $encoding = null)
    {}

    public static function mb_detect_encoding($str, $encodingList = null, $strict = false)
    {}

    public static function mb_detect_order($encodingList = null)
    {}

    public static function mb_strlen($s, $encoding = null)
    {}

    public static function mb_strpos($haystack, $needle, $offset = 0, $encoding = null)
    {}

    public static function mb_strrpos($haystack, $needle, $offset = 0, $encoding = null)
    {}

    public static function mb_str_split($string, $split_length = 1, $encoding = null)
    {}

    public static function mb_strtolower($s, $encoding = null)
    {}

    public static function mb_strtoupper($s, $encoding = null)
    {}

    public static function mb_substitute_character($c = null)
    {}

    public static function mb_substr($s, $start, $length = null, $encoding = null)
    {}

    public static function mb_stripos($haystack, $needle, $offset = 0, $encoding = null)
    {}

    public static function mb_stristr($haystack, $needle, $part = false, $encoding = null)
    {}

    public static function mb_strrchr($haystack, $needle, $part = false, $encoding = null)
    {}

    public static function mb_strrichr($haystack, $needle, $part = false, $encoding = null)
    {}

    public static function mb_strripos($haystack, $needle, $offset = 0, $encoding = null)
    {}

    public static function mb_strstr($haystack, $needle, $part = false, $encoding = null)
    {}

    public static function mb_get_info($type = 'all')
    {}

    public static function mb_http_input($type = '')
    {}

    public static function mb_http_output($encoding = null)
    {}

    public static function mb_strwidth($s, $encoding = null)
    {}

    public static function mb_substr_count($haystack, $needle, $encoding = null)
    {}

    public static function mb_output_handler($contents, $status)
    {}

    public static function mb_chr($code, $encoding = null)
    {}

    public static function mb_ord($s, $encoding = null)
    {}

    /** @return string|false */
    public static function mb_scrub(?string $string, ?string $encoding = null): string
    {}

    /** @return string|false */
    public static function mb_str_pad(string $string, int $length, string $pad_string = ' ', int $pad_type = \STR_PAD_RIGHT, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_ucfirst(string $string, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_lcfirst(string $string, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_trim(string $string, ?string $characters = null, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_ltrim(string $string, ?string $characters = null, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_rtrim(string $string, ?string $characters = null, ?string $encoding = null)
    {}

    /**
     * Finds the position of $needle in $haystack like native mbstring.
     *
     * @return int|false
     */
    private static function find(string $function, $haystack, $needle, $offset, $encoding, bool $reverse, bool $fold)
    {}

    /**
     * Returns the part of $haystack before or from $needle like native mbstring.
     *
     * @return string|false
     */
    private static function findPart(string $function, $haystack, $needle, $part, $encoding, bool $reverse, bool $fold)
    {}

    /**
     * @return array{string, string, bool, int}|null The haystack and the needle to search, whether offsets in them count bytes rather than UTF-8 characters, and how many bytes of ill-formed sequences iconv() dropped from the haystack
     */
    private static function prepareSearch(string $haystack, string $needle, string $encoding, bool $fold, bool $markIllFormed)
    {}

    private static function searchEncoding($encoding, bool $fold): string
    {}

    /**
     * Converts $s to UTF-8 to search it, ending with "\xFF", a byte that is invalid in UTF-8, when its last sequence is truncated, like native mbstring.
     *
     * @param int $dropped Set to the number of bytes of the ill-formed sequences that iconv() dropped elsewhere
     *
     * @return string|false False when the encoding is unknown
     */
    private static function utf8(string $s, string $encoding, &$dropped = 0)
    {}

    /**
     * Replaces each maximal subpart of ill-formed UTF-8 sequences with "\xFF", like native mbstring before searching case-insensitively.
     */
    private static function markIllFormedUtf8(string $s): string
    {}

    /**
     * Folds the case of UTF-8 $s like the simple case folding of native mbstring.
     */
    private static function foldCase(string $s): string
    {}

    /**
     * Counts the characters of UTF-8 $s like native mbstring, even when ill-formed: one per byte that is not a continuation byte.
     */
    private static function utf8Length(string $s): int
    {}

    /**
     * @return int|false The offset in bytes of the character at $offset, false when it is out of range, like native mbstring
     */
    private static function utf8Offset(string $s, int $offset, bool $bytes)
    {}

    private static function html_encoding_callback(array $m)
    {}

    private static function title_case(array $s)
    {}

    private static function getData($file)
    {}

    private static function getEncoding($encoding)
    {}

    private static function iconv($fromEncoding, $toEncoding, $s)
    {}

    /** @return string|false */
    private static function mb_internal_trim(string $regex, string $string, ?string $characters, ?string $encoding, string $function)
    {}

    /**
     * Trims ill-formed UTF-8 like mbstring: each maximal subpart of an ill-formed
     * sequence counts as one character, and the string is returned as is when
     * nothing is trimmed, else re-encoded with the substitute character.
     */
    private static function mb_trim_invalid_utf8(string $string, ?string $characters, string $function): string
    {}

    private static function scrubUtf8(string $s): string
    {}

    /**
     * @param true[] $referenceIds The ids of the references followed to reach $array
     */
    private static function convertArray(array $array, $toEncoding, $fromEncoding, array $referenceIds = []): array
    {}

    /**
     * Returns the error native mbstring throws for these encoding arguments, if any.
     */
    private static function getConversionError(string $function, int $argument, $toEncoding, $fromEncoding): ?string
    {}

    private static function isConvertibleEncoding(string $encoding): bool
    {}

    private static function assertEncoding(string $encoding, string $errorFormat): bool
    {}

    private static function assertCodepointEncoding(string $encoding, string $function): bool
    {}
}
