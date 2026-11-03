<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\String;

use Symfony\Component\String\Exception\ExceptionInterface;
use Symfony\Component\String\Exception\InvalidArgumentException;
use Symfony\Component\String\Exception\RuntimeException;

/**
 * Represents a string of abstract Unicode characters.
 *
 * Unicode defines 3 types of "characters" (bytes, code points and grapheme clusters).
 * This class is the abstract type to use as a type-hint when the logic you want to
 * implement is Unicode-aware but doesn't care about code points vs grapheme clusters.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @throws ExceptionInterface
 */
abstract class AbstractUnicodeString extends AbstractString
{
    public const NFC = \Normalizer::NFC;
    public const NFD = \Normalizer::NFD;
    public const NFKC = \Normalizer::NFKC;
    public const NFKD = \Normalizer::NFKD;

    // all ASCII letters sorted by typical frequency of occurrence
    private const ASCII = "\x20\x65\x69\x61\x73\x6E\x74\x72\x6F\x6C\x75\x64\x5D\x5B\x63\x6D\x70\x27\x0A\x67\x7C\x68\x76\x2E\x66\x62\x2C\x3A\x3D\x2D\x71\x31\x30\x43\x32\x2A\x79\x78\x29\x28\x4C\x39\x41\x53\x2F\x50\x22\x45\x6A\x4D\x49\x6B\x33\x3E\x35\x54\x3C\x44\x34\x7D\x42\x7B\x38\x46\x77\x52\x36\x37\x55\x47\x4E\x3B\x4A\x7A\x56\x23\x48\x4F\x57\x5F\x26\x21\x4B\x3F\x58\x51\x25\x59\x5C\x09\x5A\x2B\x7E\x5E\x24\x40\x60\x7F\x00\x01\x02\x03\x04\x05\x06\x07\x08\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

    // the subset of folded case mappings that is not in lower case mappings
    private const FOLD_FROM = ['İ', 'µ', 'ſ', "\xCD\x85", 'ς', 'ϐ', 'ϑ', 'ϕ', 'ϖ', 'ϰ', 'ϱ', 'ϵ', 'ẛ', "\xE1\xBE\xBE", 'ß', 'ŉ', 'ǰ', 'ΐ', 'ΰ', 'և', 'ẖ', 'ẗ', 'ẘ', 'ẙ', 'ẚ', 'ẞ', 'ὐ', 'ὒ', 'ὔ', 'ὖ', 'ᾀ', 'ᾁ', 'ᾂ', 'ᾃ', 'ᾄ', 'ᾅ', 'ᾆ', 'ᾇ', 'ᾈ', 'ᾉ', 'ᾊ', 'ᾋ', 'ᾌ', 'ᾍ', 'ᾎ', 'ᾏ', 'ᾐ', 'ᾑ', 'ᾒ', 'ᾓ', 'ᾔ', 'ᾕ', 'ᾖ', 'ᾗ', 'ᾘ', 'ᾙ', 'ᾚ', 'ᾛ', 'ᾜ', 'ᾝ', 'ᾞ', 'ᾟ', 'ᾠ', 'ᾡ', 'ᾢ', 'ᾣ', 'ᾤ', 'ᾥ', 'ᾦ', 'ᾧ', 'ᾨ', 'ᾩ', 'ᾪ', 'ᾫ', 'ᾬ', 'ᾭ', 'ᾮ', 'ᾯ', 'ᾲ', 'ᾳ', 'ᾴ', 'ᾶ', 'ᾷ', 'ᾼ', 'ῂ', 'ῃ', 'ῄ', 'ῆ', 'ῇ', 'ῌ', 'ῒ', 'ῖ', 'ῗ', 'ῢ', 'ῤ', 'ῦ', 'ῧ', 'ῲ', 'ῳ', 'ῴ', 'ῶ', 'ῷ', 'ῼ', 'ﬀ', 'ﬁ', 'ﬂ', 'ﬃ', 'ﬄ', 'ﬅ', 'ﬆ', 'ﬓ', 'ﬔ', 'ﬕ', 'ﬖ', 'ﬗ'];
    private const FOLD_TO = ['i̇', 'μ', 's', 'ι', 'σ', 'β', 'θ', 'φ', 'π', 'κ', 'ρ', 'ε', 'ṡ', 'ι', 'ss', 'ʼn', 'ǰ', 'ΐ', 'ΰ', 'եւ', 'ẖ', 'ẗ', 'ẘ', 'ẙ', 'aʾ', 'ss', 'ὐ', 'ὒ', 'ὔ', 'ὖ', 'ἀι', 'ἁι', 'ἂι', 'ἃι', 'ἄι', 'ἅι', 'ἆι', 'ἇι', 'ἀι', 'ἁι', 'ἂι', 'ἃι', 'ἄι', 'ἅι', 'ἆι', 'ἇι', 'ἠι', 'ἡι', 'ἢι', 'ἣι', 'ἤι', 'ἥι', 'ἦι', 'ἧι', 'ἠι', 'ἡι', 'ἢι', 'ἣι', 'ἤι', 'ἥι', 'ἦι', 'ἧι', 'ὠι', 'ὡι', 'ὢι', 'ὣι', 'ὤι', 'ὥι', 'ὦι', 'ὧι', 'ὠι', 'ὡι', 'ὢι', 'ὣι', 'ὤι', 'ὥι', 'ὦι', 'ὧι', 'ὰι', 'αι', 'άι', 'ᾶ', 'ᾶι', 'αι', 'ὴι', 'ηι', 'ήι', 'ῆ', 'ῆι', 'ηι', 'ῒ', 'ῖ', 'ῗ', 'ῢ', 'ῤ', 'ῦ', 'ῧ', 'ὼι', 'ωι', 'ώι', 'ῶ', 'ῶι', 'ωι', 'ff', 'fi', 'fl', 'ffi', 'ffl', 'st', 'st', 'մն', 'մե', 'մի', 'վն', 'մխ'];

    // the subset of https://github.com/unicode-org/cldr/blob/master/common/transforms/Latin-ASCII.xml that is not in NFKD
    private const TRANSLIT_FROM = ['Æ', 'Ð', 'Ø', 'Þ', 'ß', 'æ', 'ð', 'ø', 'þ', 'Đ', 'đ', 'Ħ', 'ħ', 'ı', 'ĸ', 'Ŀ', 'ŀ', 'Ł', 'ł', 'ŉ', 'Ŋ', 'ŋ', 'Œ', 'œ', 'Ŧ', 'ŧ', 'ƀ', 'Ɓ', 'Ƃ', 'ƃ', 'Ƈ', 'ƈ', 'Ɖ', 'Ɗ', 'Ƌ', 'ƌ', 'Ɛ', 'Ƒ', 'ƒ', 'Ɠ', 'ƕ', 'Ɩ', 'Ɨ', 'Ƙ', 'ƙ', 'ƚ', 'Ɲ', 'ƞ', 'Ƣ', 'ƣ', 'Ƥ', 'ƥ', 'ƫ', 'Ƭ', 'ƭ', 'Ʈ', 'Ʋ', 'Ƴ', 'ƴ', 'Ƶ', 'ƶ', 'Ǆ', 'ǅ', 'ǆ', 'Ǥ', 'ǥ', 'ȡ', 'Ȥ', 'ȥ', 'ȴ', 'ȵ', 'ȶ', 'ȷ', 'ȸ', 'ȹ', 'Ⱥ', 'Ȼ', 'ȼ', 'Ƚ', 'Ⱦ', 'ȿ', 'ɀ', 'Ƀ', 'Ʉ', 'Ɇ', 'ɇ', 'Ɉ', 'ɉ', 'Ɍ', 'ɍ', 'Ɏ', 'ɏ', 'ɓ', 'ɕ', 'ɖ', 'ɗ', 'ɛ', 'ɟ', 'ɠ', 'ɡ', 'ɢ', 'ɦ', 'ɧ', 'ɨ', 'ɪ', 'ɫ', 'ɬ', 'ɭ', 'ɱ', 'ɲ', 'ɳ', 'ɴ', 'ɶ', 'ɼ', 'ɽ', 'ɾ', 'ʀ', 'ʂ', 'ʈ', 'ʉ', 'ʋ', 'ʏ', 'ʐ', 'ʑ', 'ʙ', 'ʛ', 'ʜ', 'ʝ', 'ʟ', 'ʠ', 'ʣ', 'ʥ', 'ʦ', 'ʪ', 'ʫ', 'ᴀ', 'ᴁ', 'ᴃ', 'ᴄ', 'ᴅ', 'ᴆ', 'ᴇ', 'ᴊ', 'ᴋ', 'ᴌ', 'ᴍ', 'ᴏ', 'ᴘ', 'ᴛ', 'ᴜ', 'ᴠ', 'ᴡ', 'ᴢ', 'ᵫ', 'ᵬ', 'ᵭ', 'ᵮ', 'ᵯ', 'ᵰ', 'ᵱ', 'ᵲ', 'ᵳ', 'ᵴ', 'ᵵ', 'ᵶ', 'ᵺ', 'ᵻ', 'ᵽ', 'ᵾ', 'ᶀ', 'ᶁ', 'ᶂ', 'ᶃ', 'ᶄ', 'ᶅ', 'ᶆ', 'ᶇ', 'ᶈ', 'ᶉ', 'ᶊ', 'ᶌ', 'ᶍ', 'ᶎ', 'ᶏ', 'ᶑ', 'ᶒ', 'ᶓ', 'ᶖ', 'ᶙ', 'ẚ', 'ẜ', 'ẝ', 'ẞ', 'Ỻ', 'ỻ', 'Ỽ', 'ỽ', 'Ỿ', 'ỿ', '©', '®', '₠', '₢', '₣', '₤', '₧', '₺', '₹', 'ℌ', '℞', '㎧', '㎮', '㏆', '㏗', '㏞', '㏟', '¼', '½', '¾', '⅓', '⅔', '⅕', '⅖', '⅗', '⅘', '⅙', '⅚', '⅛', '⅜', '⅝', '⅞', '⅟', '〇', '‘', '’', '‚', '‛', '“', '”', '„', '‟', '′', '″', '〝', '〞', '«', '»', '‹', '›', '‐', '‑', '‒', '–', '—', '―', '︱', '︲', '﹘', '‖', '⁄', '⁅', '⁆', '⁎', '、', '。', '〈', '〉', '《', '》', '〔', '〕', '〘', '〙', '〚', '〛', '︑', '︒', '︹', '︺', '︽', '︾', '︿', '﹀', '﹑', '﹝', '﹞', '｟', '｠', '｡', '､', '×', '÷', '−', '∕', '∖', '∣', '∥', '≪', '≫', '⦅', '⦆'];
    private const TRANSLIT_TO = ['AE', 'D', 'O', 'TH', 'ss', 'ae', 'd', 'o', 'th', 'D', 'd', 'H', 'h', 'i', 'q', 'L', 'l', 'L', 'l', '\'n', 'N', 'n', 'OE', 'oe', 'T', 't', 'b', 'B', 'B', 'b', 'C', 'c', 'D', 'D', 'D', 'd', 'E', 'F', 'f', 'G', 'hv', 'I', 'I', 'K', 'k', 'l', 'N', 'n', 'OI', 'oi', 'P', 'p', 't', 'T', 't', 'T', 'V', 'Y', 'y', 'Z', 'z', 'DZ', 'Dz', 'dz', 'G', 'g', 'd', 'Z', 'z', 'l', 'n', 't', 'j', 'db', 'qp', 'A', 'C', 'c', 'L', 'T', 's', 'z', 'B', 'U', 'E', 'e', 'J', 'j', 'R', 'r', 'Y', 'y', 'b', 'c', 'd', 'd', 'e', 'j', 'g', 'g', 'G', 'h', 'h', 'i', 'I', 'l', 'l', 'l', 'm', 'n', 'n', 'N', 'OE', 'r', 'r', 'r', 'R', 's', 't', 'u', 'v', 'Y', 'z', 'z', 'B', 'G', 'H', 'j', 'L', 'q', 'dz', 'dz', 'ts', 'ls', 'lz', 'A', 'AE', 'B', 'C', 'D', 'D', 'E', 'J', 'K', 'L', 'M', 'O', 'P', 'T', 'U', 'V', 'W', 'Z', 'ue', 'b', 'd', 'f', 'm', 'n', 'p', 'r', 'r', 's', 't', 'z', 'th', 'I', 'p', 'U', 'b', 'd', 'f', 'g', 'k', 'l', 'm', 'n', 'p', 'r', 's', 'v', 'x', 'z', 'a', 'd', 'e', 'e', 'i', 'u', 'a', 's', 's', 'SS', 'LL', 'll', 'V', 'v', 'Y', 'y', '(C)', '(R)', 'CE', 'Cr', 'Fr.', 'L.', 'Pts', 'TL', 'Rs', 'x', 'Rx', 'm/s', 'rad/s', 'C/kg', 'pH', 'V/m', 'A/m', ' 1/4', ' 1/2', ' 3/4', ' 1/3', ' 2/3', ' 1/5', ' 2/5', ' 3/5', ' 4/5', ' 1/6', ' 5/6', ' 1/8', ' 3/8', ' 5/8', ' 7/8', ' 1/', '0', '\'', '\'', ',', '\'', '"', '"', ',,', '"', '\'', '"', '"', '"', '<<', '>>', '<', '>', '-', '-', '-', '-', '-', '-', '-', '-', '-', '||', '/', '[', ']', '*', ',', '.', '<', '>', '<<', '>>', '[', ']', '[', ']', '[', ']', ',', '.', '[', ']', '<<', '>>', '<', '>', ',', '[', ']', '((', '))', '.', ',', '*', '/', '-', '/', '\\', '|', '||', '<<', '>>', '((', '))'];

    // \p{ExtPict} requires PCRE2 10.40; U+1F1E6-1F1FF are regional indicators, #*0-9 keycap bases
    private const EMOJI_REGEX = \PCRE_VERSION_MAJOR < 10 || \PCRE_VERSION_MAJOR === 10 && \PCRE_VERSION_MINOR < 40 ? '/[\p{So}#*0-9]/u' : '/[\p{ExtPict}\x{1F1E6}-\x{1F1FF}#*0-9]/u';

    private static array $transliterators = [];
    private static array $tableZero;
    private static array $tableWide;

    public static function fromCodePoints(int ...$codes): static
    {}

    /**
     * Generic UTF-8 to ASCII transliteration.
     *
     * Install the intl extension for best results.
     *
     * @param string[]|\Transliterator[]|\Closure[] $rules See "*-Latin" rules from Transliterator::listIDs()
     */
    public function ascii(array $rules = []): self
    {}

    public function camel(): static
    {}

    /**
     * @return int[]
     */
    public function codePointsAt(int $offset): array
    {}

    public function folded(bool $compat = true): static
    {}

    public function join(array $strings, ?string $lastGlue = null): static
    {}

    public function lower(): static
    {}

    /**
     * @param string $locale In the format language_region (e.g. tr_TR)
     */
    public function localeLower(string $locale): static
    {}

    public function match(string $regexp, int $flags = 0, int $offset = 0): array
    {}

    public function normalize(int $form = self::NFC): static
    {}

    public function padBoth(int $length, string $padStr = ' '): static
    {}

    public function padEnd(int $length, string $padStr = ' '): static
    {}

    public function padStart(int $length, string $padStr = ' '): static
    {}

    /**
     * @param-immediately-invoked-callable $to
     */
    public function replaceMatches(string $fromRegexp, string|callable $to): static
    {}

    public function reverse(): static
    {}

    public function snake(): static
    {}

    public function title(bool $allWords = false): static
    {}

    /**
     * @param string $locale In the format language_region (e.g. tr_TR)
     */
    public function localeTitle(string $locale): static
    {}

    public function trim(string $chars = " \t\n\r\0\x0B\x0C\u{A0}\u{FEFF}"): static
    {}

    public function trimEnd(string $chars = " \t\n\r\0\x0B\x0C\u{A0}\u{FEFF}"): static
    {}

    public function trimPrefix($prefix): static
    {}

    public function trimStart(string $chars = " \t\n\r\0\x0B\x0C\u{A0}\u{FEFF}"): static
    {}

    public function trimSuffix($suffix): static
    {}

    public function upper(): static
    {}

    /**
     * @param string $locale In the format language_region (e.g. tr_TR)
     */
    public function localeUpper(string $locale): static
    {}

    public function width(bool $ignoreAnsiDecoration = true): int
    {}

    /**
     * Quotes $chars for a character class, reading "x..y" as a range of code points like trim() does with bytes.
     */
    private static function quoteCharRanges(string $chars): string
    {}

    private function pad(int $len, self $pad, int $type): static
    {}

    /**
     * Based on https://github.com/jquast/wcwidth, a Python implementation of https://www.cl.cam.ac.uk/~mgk25/ucs/wcwidth.c.
     */
    private function wcswidth(string $string): int
    {}

    private function getLocaleTransliterator(string $locale, string $id): ?\Transliterator
    {}
}
