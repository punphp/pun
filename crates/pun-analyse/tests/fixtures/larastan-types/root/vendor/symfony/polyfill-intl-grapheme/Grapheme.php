<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Intl\Grapheme;

\define('SYMFONY_GRAPHEME_CLUSTER_RX', ((float) \PCRE_VERSION >= 10.44) ? '\X' : Grapheme::GRAPHEME_CLUSTER_RX);

/**
 * Partial intl implementation in pure PHP.
 *
 * Implemented:
 * - grapheme_extract  - Extract a sequence of grapheme clusters from a text buffer, which must be encoded in UTF-8
 * - grapheme_stripos  - Find position (in grapheme units) of first occurrence of a case-insensitive string
 * - grapheme_stristr  - Returns part of haystack string from the first occurrence of case-insensitive needle to the end of haystack
 * - grapheme_strlen   - Get string length in grapheme units
 * - grapheme_strpos   - Find position (in grapheme units) of first occurrence of a string
 * - grapheme_strripos - Find position (in grapheme units) of last occurrence of a case-insensitive string
 * - grapheme_strrpos  - Find position (in grapheme units) of last occurrence of a string
 * - grapheme_strstr   - Returns part of haystack string from the first occurrence of needle to the end of haystack
 * - grapheme_substr   - Return part of a string
 * - grapheme_str_split - Splits a string into an array of individual or chunks of graphemes
 * - grapheme_levenshtein - Calculate the grapheme-unit Levenshtein distance between two strings
 * - grapheme_strrev - Reverse a string by grapheme clusters
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @internal
 */
final class Grapheme
{
    // (CRLF|RI RI?|(([ZWNJ-ZWJ]|T+|L*(LV?V+|LV|LVT)T*|L+|[^Control])[Extend]*)(ZWJ([ZWNJ-ZWJ]|T+|L*(LV?V+|LV|LVT)T*|L+|[^Control])[Extend]*)*|[Control])
    // This regular expression is a work around for http://bugs.exim.org/1279
    public const GRAPHEME_CLUSTER_RX = '(?:\r\n|[\x{1F1E6}-\x{1F1FF}][\x{1F1E6}-\x{1F1FF}]?|(?:[ -~\x{200C}\x{200D}]|[ᆨ-ᇹ]+|[ᄀ-ᅟ]*(?:[가개갸걔거게겨계고과괘괴교구궈궤귀규그긔기까깨꺄꺠꺼께껴꼐꼬꽈꽤꾀꾜꾸꿔꿰뀌뀨끄끠끼나내냐냬너네녀녜노놔놰뇌뇨누눠눼뉘뉴느늬니다대댜댸더데뎌뎨도돠돼되됴두둬뒈뒤듀드듸디따때땨떄떠떼뗘뗴또똬뙈뙤뚀뚜뚸뛔뛰뜌뜨띄띠라래랴럐러레려례로롸뢔뢰료루뤄뤠뤼류르릐리마매먀먜머메며몌모뫄뫠뫼묘무뭐뭬뮈뮤므믜미바배뱌뱨버베벼볘보봐봬뵈뵤부붜붸뷔뷰브븨비빠빼뺘뺴뻐뻬뼈뼤뽀뽜뽸뾔뾰뿌뿨쀄쀠쀼쁘쁴삐사새샤섀서세셔셰소솨쇄쇠쇼수숴쉐쉬슈스싀시싸쌔쌰썌써쎄쎠쎼쏘쏴쐐쐬쑈쑤쒀쒜쒸쓔쓰씌씨아애야얘어에여예오와왜외요우워웨위유으의이자재쟈쟤저제져졔조좌좨죄죠주줘줴쥐쥬즈즤지짜째쨔쨰쩌쩨쪄쪠쪼쫘쫴쬐쬬쭈쭤쮀쮜쮸쯔쯰찌차채챠챼처체쳐쳬초촤쵀최쵸추춰췌취츄츠츼치카캐캬컈커케켜켸코콰쾌쾨쿄쿠쿼퀘퀴큐크킈키타태탸턔터테텨톄토톼퇘퇴툐투퉈퉤튀튜트틔티파패퍄퍠퍼페펴폐포퐈퐤푀표푸풔풰퓌퓨프픠피하해햐햬허헤혀혜호화홰회효후훠훼휘휴흐희히]?[ᅠ-ᆢ]+|[가-힣])[ᆨ-ᇹ]*|[ᄀ-ᅟ]+|[^\p{Cc}\p{Cf}\p{Zl}\p{Zp}])[\p{Mn}\p{Mc}\p{Me}\x{09BE}\x{09D7}\x{0B3E}\x{0B57}\x{0BBE}\x{0BD7}\x{0CC2}\x{0CD5}\x{0CD6}\x{0D3E}\x{0D57}\x{0DCF}\x{0DDF}\x{200C}\x{1D165}\x{1D16E}-\x{1D172}\x{1F3FB}-\x{1F3FF}\x{FE0E}-\x{FE0F}\x{E0020}-\x{E007F}]*(?:\x{200D}(?:[\x{1F1E6}-\x{1F1FF}]|[ -~\x{200C}\x{200D}]|[ᆨ-ᇹ]+|[ᄀ-ᅟ]*(?:[가-힣]?[ᅠ-ᆢ]+|[가-힣])[ᆨ-ᇹ]*|[ᄀ-ᅟ]+|[^\p{Cc}\p{Cf}\p{Zl}\p{Zp}])[\p{Mn}\p{Mc}\p{Me}\x{09BE}\x{09D7}\x{0B3E}\x{0B57}\x{0BBE}\x{0BD7}\x{0CC2}\x{0CD5}\x{0CD6}\x{0D3E}\x{0D57}\x{0DCF}\x{0DDF}\x{200C}\x{1D165}\x{1D16E}-\x{1D172}\x{1F3FB}-\x{1F3FF}\x{FE0E}-\x{FE0F}\x{E0020}-\x{E007F}]*)*|[\p{Cc}\p{Cf}\p{Zl}\p{Zp}])';

    // Table 3-7 of the Unicode standard for the well-formed byte sequences (group 1),
    // then the maximal subparts an ill-formed sequence is made of (group 2).
    private const UTF8_CHAR_OR_SUBPART_RX = '/([\x00-\x7F]|[\xC2-\xDF][\x80-\xBF]|\xE0[\xA0-\xBF][\x80-\xBF]|[\xE1-\xEC][\x80-\xBF]{2}|\xED[\x80-\x9F][\x80-\xBF]|[\xEE-\xEF][\x80-\xBF]{2}|\xF0[\x90-\xBF][\x80-\xBF]{2}|[\xF1-\xF3][\x80-\xBF]{3}|\xF4[\x80-\x8F][\x80-\xBF]{2})|(\xF0[\x90-\xBF][\x80-\xBF]?|[\xF1-\xF3][\x80-\xBF]{1,2}|\xF4[\x80-\x8F][\x80-\xBF]?|\xE0[\xA0-\xBF]|[\xE1-\xEC][\x80-\xBF]|\xED[\x80-\x9F]|[\xEE-\xEF][\x80-\xBF]|[\x80-\xFF])/s';

    private const CASE_FOLD = [
        ['µ', 'ſ', "\xCD\x85", 'ς', "\xCF\x90", "\xCF\x91", "\xCF\x95", "\xCF\x96", "\xCF\xB0", "\xCF\xB1", "\xCF\xB5", "\xE1\xBA\x9B", "\xE1\xBE\xBE"],
        ['μ', 's', 'ι',        'σ', 'β',        'θ',        'φ',        'π',        'κ',        'ρ',        'ε',        "\xE1\xB9\xA1", 'ι'],
    ];

    // The characters that extend a grapheme cluster and that ICU 74 ignores when it compares strings
    private const IGNORABLE_RX = '/^[\x{034F}\x{0488}-\x{0489}\x{0591}-\x{05AF}\x{05BD}\x{05C4}-\x{05C5}\x{0610}-\x{061A}\x{06D6}-\x{06DC}\x{06DF}-\x{06E4}\x{06E7}-\x{06E8}\x{06EA}-\x{06ED}\x{0740}\x{0743}-\x{0744}\x{0747}-\x{074A}\x{0898}-\x{089D}\x{08CA}-\x{08E1}\x{08EA}-\x{08EF}\x{08F3}\x{0951}-\x{0952}\x{0F18}-\x{0F19}\x{0F35}\x{0F37}\x{0F3E}-\x{0F3F}\x{0F86}-\x{0F87}\x{0FC6}\x{17B4}-\x{17B5}\x{17D3}\x{180B}-\x{180D}\x{180F}\x{1A7F}\x{1B6B}-\x{1B73}\x{1CD0}-\x{1CD2}\x{1CD4}-\x{1CE8}\x{1CF4}\x{1CF7}-\x{1CF9}\x{200C}-\x{200D}\x{2D7F}\x{A670}-\x{A672}\x{A8E0}-\x{A8F1}\x{FE00}-\x{FE0F}\x{FE21}\x{FE23}-\x{FE26}\x{FE28}\x{FE2A}-\x{FE2D}\x{FE2F}\x{102E0}\x{10EFD}-\x{10EFF}\x{11366}-\x{1136C}\x{11370}-\x{11374}\x{13440}\x{13447}-\x{13455}\x{16FE4}\x{1CF00}-\x{1CF2D}\x{1CF30}-\x{1CF46}\x{1D165}-\x{1D169}\x{1D16D}-\x{1D172}\x{1D17B}-\x{1D182}\x{1D185}-\x{1D18B}\x{1D1AA}-\x{1D1AD}\x{1D242}-\x{1D244}\x{1DA00}-\x{1DA36}\x{1DA3B}-\x{1DA6C}\x{1DA75}\x{1DA84}\x{1DA9B}-\x{1DA9F}\x{1DAA1}-\x{1DAAF}\x{1E8D0}-\x{1E8D6}\x{E0020}-\x{E007F}\x{E0100}-\x{E01EF}]*+\z/u';

    // indexed by the $mode argument of grapheme_position()
    private const POSITION_FUNCTIONS = ['grapheme_strpos', 'grapheme_stripos', 'grapheme_strrpos', 'grapheme_strripos'];

    public static function grapheme_extract($s, $size, $type = \GRAPHEME_EXTR_COUNT, $start = 0, &$next = 0)
    {}

    public static function grapheme_strlen($s)
    {}

    public static function grapheme_substr($s, $start, $len = null)
    {}

    public static function grapheme_strpos($s, $needle, $offset = 0)
    {}

    public static function grapheme_stripos($s, $needle, $offset = 0)
    {}

    public static function grapheme_strrpos($s, $needle, $offset = 0)
    {}

    public static function grapheme_strripos($s, $needle, $offset = 0)
    {}

    public static function grapheme_stristr($s, $needle, $beforeNeedle = false)
    {}

    public static function grapheme_strstr($s, $needle, $beforeNeedle = false)
    {}

    public static function grapheme_str_split($s, $len = 1)
    {}

    public static function grapheme_levenshtein($s1, $s2, $insertion_cost = 1, $replacement_cost = 1, $deletion_cost = 1, $locale = '')
    {}

    private static function grapheme_position($s, $needle, $offset, $mode)
    {}

    /**
     * Returns the position in grapheme units of the bytes from $start to $end,
     * or false when they split a grapheme cluster on characters that are not ignorable.
     */
    private static function graphemeIndex($s, $start, $end)
    {}

    private static function isAsciiBoundary($s, $i)
    {}

    public static function grapheme_strrev(string $string)
    {}

    private static function isUtf8(?string $s)
    {}

    /**
     * Splits a string that is not valid UTF-8 into grapheme clusters.
     *
     * The intl extension keeps the ill-formed bytes and breaks around them the
     * way it would around the U+FFFD each maximal subpart stands for. Standing
     * one in for real makes the cluster regexp applicable, then each cluster is
     * mapped back to the bytes it was made of.
     */
    private static function splitIllFormed($s)
    {}
}
