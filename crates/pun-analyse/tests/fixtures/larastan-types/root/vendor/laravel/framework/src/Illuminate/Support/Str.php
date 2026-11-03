<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use League\CommonMark\Environment\Environment;
use League\CommonMark\Extension\GithubFlavoredMarkdownExtension;
use League\CommonMark\Extension\InlinesOnly\InlinesOnlyExtension;
use League\CommonMark\GithubFlavoredMarkdownConverter;
use League\CommonMark\MarkdownConverter;
use Ramsey\Uuid\Codec\TimestampFirstCombCodec;
use Ramsey\Uuid\Exception\InvalidUuidStringException;
use Ramsey\Uuid\Generator\CombGenerator;
use Ramsey\Uuid\Rfc4122\FieldsInterface;
use Ramsey\Uuid\Uuid;
use Ramsey\Uuid\UuidFactory;
use Symfony\Component\Uid\Ulid;
use Throwable;
use Traversable;
use voku\helper\ASCII;

class Str
{
    use Macroable;

    /**
     * The list of characters that are considered "invisible" in strings.
     *
     * @var string
     */
    const INVISIBLE_CHARACTERS = '\x{0009}\x{0020}\x{00A0}\x{00AD}\x{034F}\x{061C}\x{115F}\x{1160}\x{17B4}\x{17B5}\x{180E}\x{2000}\x{2001}\x{2002}\x{2003}\x{2004}\x{2005}\x{2006}\x{2007}\x{2008}\x{2009}\x{200A}\x{200B}\x{200C}\x{200D}\x{200E}\x{200F}\x{202F}\x{205F}\x{2060}\x{2061}\x{2062}\x{2063}\x{2064}\x{2065}\x{206A}\x{206B}\x{206C}\x{206D}\x{206E}\x{206F}\x{3000}\x{2800}\x{3164}\x{FEFF}\x{FFA0}\x{1D159}\x{1D173}\x{1D174}\x{1D175}\x{1D176}\x{1D177}\x{1D178}\x{1D179}\x{1D17A}\x{E0020}';

    /**
     * The cache of snake-cased words.
     *
     * @var array<string, string>
     */
    protected static $snakeCache = [];

    /**
     * The cache of camel-cased words.
     *
     * @var array<string, string>
     */
    protected static $camelCache = [];

    /**
     * The cache of studly-cased words.
     *
     * @var array<string, string>
     */
    protected static $studlyCache = [];

    /**
     * The callback that should be used to generate UUIDs.
     *
     * @var (callable(): \Ramsey\Uuid\UuidInterface)|null
     */
    protected static $uuidFactory;

    /**
     * The callback that should be used to generate ULIDs.
     *
     * @var (callable(): \Symfony\Component\Uid\Ulid)|null
     */
    protected static $ulidFactory;

    /**
     * The callback that should be used to generate random strings.
     *
     * @var (callable(int): string)|null
     */
    protected static $randomStringFactory;

    /**
     * Get a new stringable object from the given string.
     *
     * @param  string  $string
     * @return \Illuminate\Support\Stringable
     */
    public static function of($string)
    {}

    /**
     * Translate the given message and get a new stringable object.
     *
     * @param  string  $key
     * @param  array  $replace
     * @param  string|null  $locale
     * @return \Illuminate\Support\Stringable
     */
    public static function trans($key, $replace = [], $locale = null)
    {}

    /**
     * Return the remainder of a string after the first occurrence of a given value.
     *
     * @param  string  $subject
     * @param  string  $search
     * @return string
     */
    public static function after($subject, $search)
    {}

    /**
     * Return the remainder of a string after the last occurrence of a given value.
     *
     * @param  string  $subject
     * @param  string  $search
     * @return string
     */
    public static function afterLast($subject, $search)
    {}

    /**
     * Transliterate a UTF-8 value to ASCII.
     *
     * @param  string  $value
     * @param  string  $language
     * @return string
     */
    public static function ascii($value, $language = 'en')
    {}

    /**
     * Transliterate a string to its closest ASCII representation.
     *
     * @param  string  $string
     * @param  string|null  $unknown
     * @param  bool|null  $strict
     * @return string
     */
    public static function transliterate($string, $unknown = '?', $strict = false)
    {}

    /**
     * Get the portion of a string before the first occurrence of a given value.
     *
     * @param  string  $subject
     * @param  string  $search
     * @return string
     */
    public static function before($subject, $search)
    {}

    /**
     * Get the portion of a string before the last occurrence of a given value.
     *
     * @param  string  $subject
     * @param  string  $search
     * @return string
     */
    public static function beforeLast($subject, $search)
    {}

    /**
     * Get the portion of a string between two given values.
     *
     * @param  string  $subject
     * @param  string  $from
     * @param  string  $to
     * @return string
     */
    public static function between($subject, $from, $to)
    {}

    /**
     * Get the smallest possible portion of a string between two given values.
     *
     * @param  string  $subject
     * @param  string  $from
     * @param  string  $to
     * @return string
     */
    public static function betweenFirst($subject, $from, $to)
    {}

    /**
     * Convert a value to camel case.
     *
     * @param  string  $value
     * @return ($value is '' ? '' : string)
     */
    public static function camel($value)
    {}

    /**
     * Get the character at the specified index.
     *
     * @param  string  $subject
     * @param  int  $index
     * @return string|false
     */
    public static function charAt($subject, $index)
    {}

    /**
     * Remove the given string(s) if it exists at the start of the haystack.
     *
     * @param  string  $subject
     * @param  string|string[]  $needle
     * @return string
     */
    public static function chopStart($subject, $needle)
    {}

    /**
     * Remove the given string(s) if it exists at the end of the haystack.
     *
     * @param  string  $subject
     * @param  string|string[]  $needle
     * @return string
     */
    public static function chopEnd($subject, $needle)
    {}

    /**
     * Determine if a given string contains a given substring.
     *
     * @param  string  $haystack
     * @param  string|iterable<string>  $needles
     * @param  bool  $ignoreCase
     * @return ($needles is array{} ? false : ($haystack is non-empty-string ? bool : false))
     */
    public static function contains($haystack, $needles, $ignoreCase = false)
    {}

    /**
     * Determine if a given string contains all array values.
     *
     * @param  string  $haystack
     * @param  iterable<string>  $needles
     * @param  bool  $ignoreCase
     * @return ($needles is array{} ? false : ($haystack is non-empty-string ? bool : false))
     */
    public static function containsAll($haystack, $needles, $ignoreCase = false)
    {}

    /**
     * Determine if a given string doesn't contain a given substring.
     *
     * @param  string  $haystack
     * @param  string|iterable<string>  $needles
     * @param  bool  $ignoreCase
     * @return ($needles is array{} ? true : ($haystack is non-empty-string ? bool : true))
     */
    public static function doesntContain($haystack, $needles, $ignoreCase = false)
    {}

    /**
     * Convert the case of a string.
     *
     * @param  string  $string
     * @param  MB_CASE_UPPER|MB_CASE_LOWER|MB_CASE_TITLE|MB_CASE_FOLD|MB_CASE_UPPER_SIMPLE|MB_CASE_LOWER_SIMPLE|MB_CASE_TITLE_SIMPLE|MB_CASE_FOLD_SIMPLE  $mode
     * @param  string|null  $encoding
     * @return ($string is '' ? '' : string)
     */
    public static function convertCase(string $string, int $mode = MB_CASE_FOLD, ?string $encoding = 'UTF-8')
    {}

    /**
     * Get the plural form of an English word with the count prepended.
     *
     * @param  string  $value
     * @param  int|array|\Countable  $count
     * @return string
     */
    public static function counted($value, $count)
    {}

    /**
     * Replace consecutive instances of a given character with a single character in the given string.
     *
     * @param  string  $string
     * @param  array<string>|string  $characters
     * @return ($string is '' ? '' : string)
     */
    public static function deduplicate(string $string, array|string $characters = ' ')
    {}

    /**
     * Determine if a given string ends with a given substring.
     *
     * @param  string  $haystack
     * @param  string|iterable<string>  $needles
     * @return ($needles is array{} ? false : ($haystack is non-empty-string ? bool : false))
     */
    public static function endsWith($haystack, $needles)
    {}

    /**
     * Determine if a given string doesn't end with a given substring.
     *
     * @param  string  $haystack
     * @param  string|iterable<string>  $needles
     * @return ($needles is array{} ? true : ($haystack is non-empty-string ? bool : true))
     */
    public static function doesntEndWith($haystack, $needles)
    {}

    /**
     * Extracts an excerpt from text that matches the first instance of a phrase.
     *
     * @param  string  $text
     * @param  string  $phrase
     * @param  array{radius?: int|float, omission?: string}  $options
     * @return string|null
     */
    public static function excerpt($text, $phrase = '', $options = [])
    {}

    /**
     * Cap a string with a single instance of a given value.
     *
     * @param  string  $value
     * @param  string  $cap
     * @return ($value is '' ? ($cap is '' ? '' : non-empty-string) : non-empty-string)
     */
    public static function finish($value, $cap)
    {}

    /**
     * Wrap the string with the given strings.
     *
     * @param  string  $value
     * @param  string  $before
     * @param  string|null  $after
     * @return ($value is '' ? ($before is '' ? ($after is '' ? '' : ($after is null ? '' : non-empty-string)) : non-empty-string) : non-empty-string)
     */
    public static function wrap($value, $before, $after = null)
    {}

    /**
     * Unwrap the string with the given strings.
     *
     * @param  string  $value
     * @param  string  $before
     * @param  string|null  $after
     * @return string
     */
    public static function unwrap($value, $before, $after = null)
    {}

    /**
     * Determine if a given string matches a given pattern.
     *
     * @param  string|iterable<string>  $pattern
     * @param  string  $value
     * @param  bool  $ignoreCase
     * @return bool
     */
    public static function is($pattern, $value, $ignoreCase = false)
    {}

    /**
     * Determine if a given string is 7 bit ASCII.
     *
     * @param  string  $value
     * @return bool
     */
    public static function isAscii($value)
    {}

    /**
     * Determine if a given value is valid JSON.
     *
     * @param  mixed  $value
     * @return bool
     *
     * @phpstan-assert-if-true =non-empty-string $value
     */
    public static function isJson($value)
    {}

    /**
     * Determine if a given value is a valid URL.
     *
     * @param  mixed  $value
     * @param  string[]  $protocols
     * @return bool
     *
     * @phpstan-assert-if-true =non-empty-string $value
     */
    public static function isUrl($value, array $protocols = [])
    {}

    /**
     * Determine if a given value is a valid UUID.
     *
     * @param  mixed  $value
     * @param  int<0, 8>|'nil'|'max'|null  $version
     * @return bool
     *
     * @phpstan-assert-if-true =non-empty-string $value
     */
    public static function isUuid($value, $version = null)
    {}

    /**
     * Determine if a given value is a valid ULID.
     *
     * @param  mixed  $value
     * @return bool
     *
     * @phpstan-assert-if-true =non-empty-string $value
     */
    public static function isUlid($value)
    {}

    /**
     * Convert a string to kebab case.
     *
     * @param  string  $value
     * @return ($value is '' ? '' : string)
     */
    public static function kebab($value)
    {}

    /**
     * Return the length of the given string.
     *
     * @param  string  $value
     * @param  string|null  $encoding
     * @return non-negative-int
     */
    public static function length($value, $encoding = null)
    {}

    /**
     * Limit the number of characters in a string.
     *
     * @param  string  $value
     * @param  int  $limit
     * @param  string  $end
     * @param  bool  $preserveWords
     * @return string
     */
    public static function limit($value, $limit = 100, $end = '...', $preserveWords = false)
    {}

    /**
     * Convert the given string to lower-case.
     *
     * @param  string  $value
     * @return ($value is '' ? '' : non-empty-string&lowercase-string)
     */
    public static function lower($value)
    {}

    /**
     * Limit the number of words in a string.
     *
     * @param  string  $value
     * @param  int  $words
     * @param  string  $end
     * @return string
     */
    public static function words($value, $words = 100, $end = '...')
    {}

    /**
     * Converts GitHub flavored Markdown into HTML.
     *
     * @param  string  $string
     * @param  array  $options
     * @param  \League\CommonMark\Extension\ExtensionInterface[]  $extensions
     * @return ($string is '' ? '' : string)
     */
    public static function markdown($string, array $options = [], array $extensions = [])
    {}

    /**
     * Converts inline Markdown into HTML.
     *
     * @param  string  $string
     * @param  array  $options
     * @param  \League\CommonMark\Extension\ExtensionInterface[]  $extensions
     * @return ($string is '' ? '' : string)
     */
    public static function inlineMarkdown($string, array $options = [], array $extensions = [])
    {}

    /**
     * Masks a portion of a string with a repeated character.
     *
     * @param  string  $string
     * @param  string  $character
     * @param  int  $index
     * @param  int|null  $length
     * @param  string  $encoding
     * @return string
     */
    public static function mask($string, $character, $index, $length = null, $encoding = 'UTF-8')
    {}

    /**
     * Get the string matching the given pattern.
     *
     * @param  string  $pattern
     * @param  string  $subject
     * @return string
     */
    public static function match($pattern, $subject)
    {}

    /**
     * Determine if a given string matches a given pattern.
     *
     * @param  string|iterable<string>  $pattern
     * @param  string  $value
     * @return ($pattern is array{} ? false : bool)
     */
    public static function isMatch($pattern, $value)
    {}

    /**
     * Get the string matching the given pattern.
     *
     * @param  string  $pattern
     * @param  string  $subject
     * @return \Illuminate\Support\Collection
     */
    public static function matchAll($pattern, $subject)
    {}

    /**
     * Remove all non-numeric characters from a string.
     *
     * @param  string|string[]  $value
     * @return ($value is string ? string : string[])
     */
    public static function numbers($value)
    {}

    /**
     * Pad both sides of a string with another.
     *
     * @param  string  $value
     * @param  int  $length
     * @param  string  $pad
     * @return string
     */
    public static function padBoth($value, $length, $pad = ' ')
    {}

    /**
     * Pad the left side of a string with another.
     *
     * @param  string  $value
     * @param  int  $length
     * @param  string  $pad
     * @return string
     */
    public static function padLeft($value, $length, $pad = ' ')
    {}

    /**
     * Pad the right side of a string with another.
     *
     * @param  string  $value
     * @param  int  $length
     * @param  string  $pad
     * @return string
     */
    public static function padRight($value, $length, $pad = ' ')
    {}

    /**
     * Parse a Class[@]method style callback into class and method.
     *
     * @param  string  $callback
     * @param  string|null  $default
     * @return array<int, string|null>
     */
    public static function parseCallback($callback, $default = null)
    {}

    /**
     * Get the plural form of an English word.
     *
     * @param  string  $value
     * @param  int|array|\Countable  $count
     * @param  bool  $prependCount
     * @return string
     */
    public static function plural($value, $count = 2, $prependCount = false)
    {}

    /**
     * Pluralize the last word of an English, studly caps case string.
     *
     * @param  string  $value
     * @param  int|array|\Countable  $count
     * @return string
     */
    public static function pluralStudly($value, $count = 2)
    {}

    /**
     * Pluralize the last word of an English, Pascal caps case string.
     *
     * @param  string  $value
     * @param  int|array|\Countable  $count
     * @return string
     */
    public static function pluralPascal($value, $count = 2)
    {}

    /**
     * Generate a random, secure password.
     *
     * @param  int  $length
     * @param  bool  $letters
     * @param  bool  $numbers
     * @param  bool  $symbols
     * @param  bool  $spaces
     * @return ($letters is false ? ($numbers is true ? ($symbols is false ? ($spaces is false ? numeric-string : string) : string) : string) : string)
     */
    public static function password($length = 32, $letters = true, $numbers = true, $symbols = true, $spaces = false)
    {}

    /**
     * Find the multi-byte safe position of the first occurrence of a given substring in a string.
     *
     * @param  string  $haystack
     * @param  string  $needle
     * @param  int  $offset
     * @param  string|null  $encoding
     * @return ($haystack is '' ? false : ($needle is '' ? false : int|false))
     */
    public static function position($haystack, $needle, $offset = 0, $encoding = null)
    {}

    /**
     * Generate a more truly "random" alpha-numeric string.
     *
     * @param  int  $length
     * @return string
     */
    public static function random($length = 16)
    {}

    /**
     * Set the callable that will be used to generate random strings.
     *
     * @param  (callable(int): string)|null  $factory
     * @return void
     */
    public static function createRandomStringsUsing(?callable $factory = null)
    {}

    /**
     * Set the sequence that will be used to generate random strings.
     *
     * @param  string[]  $sequence
     * @param  (callable(int): string)|null  $whenMissing
     * @return void
     */
    public static function createRandomStringsUsingSequence(array $sequence, $whenMissing = null)
    {}

    /**
     * Indicate that random strings should be created normally and not using a custom factory.
     *
     * @return void
     */
    public static function createRandomStringsNormally()
    {}

    /**
     * Repeat the given string.
     *
     * @param  string  $string
     * @param  int  $times
     * @return string
     */
    public static function repeat(string $string, int $times)
    {}

    /**
     * Replace a given value in the string sequentially with an array.
     *
     * @param  string  $search
     * @param  iterable<string>  $replace
     * @param  string  $subject
     * @return string
     */
    public static function replaceArray($search, $replace, $subject)
    {}

    /**
     * Convert the given value to a string or return the given fallback on failure.
     *
     * @param  mixed  $value
     * @param  string  $fallback
     * @return string
     */
    private static function toStringOr($value, $fallback)
    {}

    /**
     * Replace the given value in the given string.
     *
     * @param  string|iterable<string>  $search
     * @param  string|iterable<string>  $replace
     * @param  string|iterable<string>  $subject
     * @param  bool  $caseSensitive
     * @return ($subject is string ? string : string[])
     */
    public static function replace($search, $replace, $subject, $caseSensitive = true)
    {}

    /**
     * Replace the given value in the given string regardless of case.
     *
     * @param  string|string[]  $search
     * @param  string|string[]  $replace
     * @param  string|string[]  $subject
     * @return string|string[]
     */
    protected static function replaceWhileIgnoringCase($search, $replace, $subject)
    {}

    /**
     * Replace the first occurrence of a given value in the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @param  string  $subject
     * @return string
     */
    public static function replaceFirst($search, $replace, $subject)
    {}

    /**
     * Replace the first occurrence of the given value if it appears at the start of the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @param  string  $subject
     * @return string
     */
    public static function replaceStart($search, $replace, $subject)
    {}

    /**
     * Replace the last occurrence of a given value in the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @param  string  $subject
     * @return string
     */
    public static function replaceLast($search, $replace, $subject)
    {}

    /**
     * Replace the last occurrence of a given value if it appears at the end of the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @param  string  $subject
     * @return string
     */
    public static function replaceEnd($search, $replace, $subject)
    {}

    /**
     * Replace the patterns matching the given regular expression.
     *
     * @param  string|string[]  $pattern
     * @param  (\Closure(array): string)|string[]|string  $replace
     * @param  string[]|string  $subject
     * @param  int  $limit
     * @return ($subject is array ? string[]|null : string|null)
     */
    public static function replaceMatches($pattern, $replace, $subject, $limit = -1)
    {}

    /**
     * Remove any occurrence of the given string in the subject.
     *
     * @param  string|iterable<string>  $search
     * @param  string|iterable<string>  $subject
     * @param  bool  $caseSensitive
     * @return string
     */
    public static function remove($search, $subject, $caseSensitive = true)
    {}

    /**
     * Reverse the given string.
     *
     * @param  string  $value
     * @return string
     */
    public static function reverse(string $value)
    {}

    /**
     * Begin a string with a single instance of a given value.
     *
     * @param  string  $value
     * @param  string  $prefix
     * @return ($value is '' ? ($prefix is '' ? '' : non-empty-string): non-empty-string)
     */
    public static function start($value, $prefix)
    {}

    /**
     * Convert the given string to upper-case.
     *
     * @param  string  $value
     * @return ($value is '' ? '' : non-empty-string&uppercase-string)
     */
    public static function upper($value)
    {}

    /**
     * Convert the given string to proper case.
     *
     * @param  string  $value
     * @return string
     */
    public static function title($value)
    {}

    /**
     * Convert the given string to proper case for each word.
     *
     * @param  string  $value
     * @return string
     */
    public static function headline($value)
    {}

    /**
     * Get the "initials" representing each word in the provided string, optionally capitalizing.
     *
     * @param  string  $value
     * @param  bool  $capitalize
     * @return string
     */
    public static function initials($value, $capitalize = false)
    {}

    /**
     * Convert the given string to APA-style title case.
     *
     * See: https://apastyle.apa.org/style-grammar-guidelines/capitalization/title-case
     *
     * @param  string  $value
     * @return string
     */
    public static function apa($value)
    {}

    /**
     * Get the singular form of an English word.
     *
     * @param  string  $value
     * @return string
     */
    public static function singular($value)
    {}

    /**
     * Generate a URL friendly "slug" from a given string.
     *
     * @param  string  $title
     * @param  string  $separator
     * @param  string|null  $language
     * @param  array<string, string>  $dictionary
     * @return string
     */
    public static function slug($title, $separator = '-', $language = 'en', $dictionary = ['@' => 'at'])
    {}

    /**
     * Convert a string to snake case.
     *
     * @param  string  $value
     * @param  string  $delimiter
     * @return string
     */
    public static function snake($value, $delimiter = '_')
    {}

    /**
     * Remove all whitespace from both ends of a string.
     *
     * @param  string  $value
     * @param  string|null  $charlist
     * @return string
     */
    public static function trim($value, $charlist = null)
    {}

    /**
     * Remove all whitespace from the beginning of a string.
     *
     * @param  string  $value
     * @param  string|null  $charlist
     * @return string
     */
    public static function ltrim($value, $charlist = null)
    {}

    /**
     * Remove all whitespace from the end of a string.
     *
     * @param  string  $value
     * @param  string|null  $charlist
     * @return string
     */
    public static function rtrim($value, $charlist = null)
    {}

    /**
     * Remove all "extra" blank space from the given string.
     *
     * @param  string  $value
     * @return string
     */
    public static function squish($value)
    {}

    /**
     * Determine if a given string starts with a given substring.
     *
     * @param  string  $haystack
     * @param  string|iterable<string>  $needles
     * @return ($needles is array{} ? false : ($haystack is non-empty-string ? bool : false))
     *
     * @phpstan-assert-if-true =non-empty-string $haystack
     */
    public static function startsWith($haystack, $needles)
    {}

    /**
     * Determine if a given string doesn't start with a given substring.
     *
     * @param  string  $haystack
     * @param  string|iterable<string>  $needles
     * @return ($needles is array{} ? true : ($haystack is non-empty-string ? bool : true))
     *
     * @phpstan-assert-if-false =non-empty-string $haystack
     */
    public static function doesntStartWith($haystack, $needles)
    {}

    /**
     * Convert a value to studly caps case.
     *
     * @param  string  $value
     * @param  bool  $normalize  When true, all-uppercase words (e.g. acronyms) are lowercased before conversion so "CBOR" becomes "Cbor" instead of "CBOR".
     * @return ($value is '' ? '' : string)
     */
    public static function studly($value, bool $normalize = false)
    {}

    /**
     * Convert a value to Pascal case.
     *
     * @param  string  $value
     * @param  bool  $normalize  When true, all-uppercase words (e.g. acronyms) are lowercased before conversion so "CBOR" becomes "Cbor" instead of "CBOR".
     * @return ($value is '' ? '' : string)
     */
    public static function pascal($value, bool $normalize = false)
    {}

    /**
     * Returns the portion of the string specified by the start and length parameters.
     *
     * @param  string  $string
     * @param  int  $start
     * @param  int|null  $length
     * @param  string  $encoding
     * @return string
     */
    public static function substr($string, $start, $length = null, $encoding = 'UTF-8')
    {}

    /**
     * Returns the number of substring occurrences.
     *
     * @param  string  $haystack
     * @param  string  $needle
     * @param  int  $offset
     * @param  int|null  $length
     * @return int
     */
    public static function substrCount($haystack, $needle, $offset = 0, $length = null)
    {}

    /**
     * Replace text within a portion of a string.
     *
     * @param  string|string[]  $string
     * @param  string|string[]  $replace
     * @param  int|int[]  $offset
     * @param  int|int[]|null  $length
     * @return string|string[]
     */
    public static function substrReplace($string, $replace, $offset = 0, $length = null)
    {}

    /**
     * Swap multiple keywords in a string with other keywords.
     *
     * @param  array<string, string>  $map
     * @param  string  $subject
     * @return string
     */
    public static function swap(array $map, $subject)
    {}

    /**
     * Take the first or last {$limit} characters of a string.
     *
     * @param  string  $string
     * @param  int  $limit
     * @return string
     */
    public static function take($string, int $limit): string
    {}

    /**
     * Convert the given string to Base64 encoding.
     *
     * @param  string  $string
     * @return ($string is '' ? '' : string)
     */
    public static function toBase64($string): string
    {}

    /**
     * Decode the given Base64 encoded string.
     *
     * @param  string  $string
     * @param  bool  $strict
     * @return ($strict is true ? ($string is '' ? '' : string|false) : ($string is '' ? '' : string))
     */
    public static function fromBase64($string, $strict = false)
    {}

    /**
     * Make a string's first character lowercase.
     *
     * @param  string  $string
     * @return ($string is '' ? '' : non-empty-string)
     */
    public static function lcfirst($string)
    {}

    /**
     * Make a string's first character uppercase.
     *
     * @param  string  $string
     * @return ($string is '' ? '' : non-empty-string)
     */
    public static function ucfirst($string)
    {}

    /**
     * Capitalize the first character of each word in a string.
     *
     * @param  string  $string
     * @param  string  $separators
     * @return ($string is '' ? '' : non-empty-string)
     */
    public static function ucwords($string, $separators = " \t\r\n\f\v")
    {}

    /**
     * Split a string into pieces by uppercase characters.
     *
     * @param  string  $string
     * @return ($string is '' ? array{} : string[])
     */
    public static function ucsplit($string)
    {}

    /**
     * Get the number of words a string contains.
     *
     * @param  string  $string
     * @param  string|null  $characters
     * @return non-negative-int
     */
    public static function wordCount($string, $characters = null)
    {}

    /**
     * Wrap a string to a given number of characters.
     *
     * @param  string  $string
     * @param  int  $characters
     * @param  string  $break
     * @param  bool  $cutLongWords
     * @return string
     */
    public static function wordWrap($string, $characters = 75, $break = "\n", $cutLongWords = false)
    {}

    /**
     * Generate a UUID (version 4).
     *
     * @return \Ramsey\Uuid\UuidInterface
     */
    public static function uuid()
    {}

    /**
     * Generate a UUID (version 7).
     *
     * @param  \DateTimeInterface|null  $time
     * @return \Ramsey\Uuid\UuidInterface
     */
    public static function uuid7($time = null)
    {}

    /**
     * Generate a time-ordered UUID.
     *
     * @return \Ramsey\Uuid\UuidInterface
     */
    public static function orderedUuid()
    {}

    /**
     * Set the callable that will be used to generate UUIDs.
     *
     * @param  (callable(): \Ramsey\Uuid\UuidInterface)|null  $factory
     * @return void
     */
    public static function createUuidsUsing(?callable $factory = null)
    {}

    /**
     * Set the sequence that will be used to generate UUIDs.
     *
     * @param  \Ramsey\Uuid\UuidInterface[]  $sequence
     * @param  (callable(): \Ramsey\Uuid\UuidInterface)|null  $whenMissing
     * @return void
     */
    public static function createUuidsUsingSequence(array $sequence, $whenMissing = null)
    {}

    /**
     * Always return the same UUID when generating new UUIDs.
     *
     * @param  (\Closure(\Ramsey\Uuid\UuidInterface): mixed)|null  $callback
     * @return \Ramsey\Uuid\UuidInterface
     */
    public static function freezeUuids(?Closure $callback = null)
    {}

    /**
     * Indicate that UUIDs should be created normally and not using a custom factory.
     *
     * @return void
     */
    public static function createUuidsNormally()
    {}

    /**
     * Generate a ULID.
     *
     * @param  \DateTimeInterface|null  $time
     * @return \Symfony\Component\Uid\Ulid
     */
    public static function ulid($time = null)
    {}

    /**
     * Indicate that ULIDs should be created normally and not using a custom factory.
     *
     * @return void
     */
    public static function createUlidsNormally()
    {}

    /**
     * Set the callable that will be used to generate ULIDs.
     *
     * @param  (callable(): \Symfony\Component\Uid\Ulid)|null  $factory
     * @return void
     */
    public static function createUlidsUsing(?callable $factory = null)
    {}

    /**
     * Set the sequence that will be used to generate ULIDs.
     *
     * @param  \Symfony\Component\Uid\Ulid[]  $sequence
     * @param  (callable(): \Symfony\Component\Uid\Ulid)|null  $whenMissing
     * @return void
     */
    public static function createUlidsUsingSequence(array $sequence, $whenMissing = null)
    {}

    /**
     * Always return the same ULID when generating new ULIDs.
     *
     * @param  (Closure(Ulid): mixed)|null  $callback
     * @return Ulid
     */
    public static function freezeUlids(?Closure $callback = null)
    {}

    /**
     * Remove all strings from the casing caches.
     *
     * @return void
     */
    public static function flushCache()
    {}

    /**
     * Return all factory functions to their default state.
     *
     * @return void
     */
    public static function resetFactoryState()
    {}
}
