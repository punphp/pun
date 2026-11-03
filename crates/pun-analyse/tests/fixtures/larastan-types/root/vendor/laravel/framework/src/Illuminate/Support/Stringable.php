<?php

namespace Illuminate\Support;

use ArrayAccess;
use Closure;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Dumpable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use JsonSerializable;
use Stringable as BaseStringable;

class Stringable implements JsonSerializable, ArrayAccess, BaseStringable
{
    use Conditionable, Dumpable, Macroable, Tappable;

    /**
     * The underlying string value.
     *
     * @var string
     */
    protected $value;

    /**
     * Create a new instance of the class.
     *
     * @param  string  $value
     */
    public function __construct($value = '')
    {}

    /**
     * Return the remainder of a string after the first occurrence of a given value.
     *
     * @param  string  $search
     * @return static
     */
    public function after($search)
    {}

    /**
     * Return the remainder of a string after the last occurrence of a given value.
     *
     * @param  string  $search
     * @return static
     */
    public function afterLast($search)
    {}

    /**
     * Append the given values to the string.
     *
     * @param  array|string  ...$values
     * @return static
     */
    public function append(...$values)
    {}

    /**
     * Append a new line to the string.
     *
     * @param  int  $count
     * @return $this
     */
    public function newLine($count = 1)
    {}

    /**
     * Transliterate a UTF-8 value to ASCII.
     *
     * @param  string  $language
     * @return static
     */
    public function ascii($language = 'en')
    {}

    /**
     * Get the trailing name component of the path.
     *
     * @param  string  $suffix
     * @return static
     */
    public function basename($suffix = '')
    {}

    /**
     * Get the character at the specified index.
     *
     * @param  int  $index
     * @return string|false
     */
    public function charAt($index)
    {}

    /**
     * Remove the given string if it exists at the start of the current string.
     *
     * @param  string|array  $needle
     * @return static
     */
    public function chopStart($needle)
    {}

    /**
     * Remove the given string if it exists at the end of the current string.
     *
     * @param  string|array  $needle
     * @return static
     */
    public function chopEnd($needle)
    {}

    /**
     * Get the basename of the class path.
     *
     * @return static
     */
    public function classBasename()
    {}

    /**
     * Get the portion of a string before the first occurrence of a given value.
     *
     * @param  string  $search
     * @return static
     */
    public function before($search)
    {}

    /**
     * Get the portion of a string before the last occurrence of a given value.
     *
     * @param  string  $search
     * @return static
     */
    public function beforeLast($search)
    {}

    /**
     * Get the portion of a string between two given values.
     *
     * @param  string  $from
     * @param  string  $to
     * @return static
     */
    public function between($from, $to)
    {}

    /**
     * Get the smallest possible portion of a string between two given values.
     *
     * @param  string  $from
     * @param  string  $to
     * @return static
     */
    public function betweenFirst($from, $to)
    {}

    /**
     * Convert a value to camel case.
     *
     * @return static
     */
    public function camel()
    {}

    /**
     * Determine if a given string contains a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  bool  $ignoreCase
     * @return bool
     */
    public function contains($needles, $ignoreCase = false)
    {}

    /**
     * Determine if a given string contains all array values.
     *
     * @param  iterable<string>  $needles
     * @param  bool  $ignoreCase
     * @return bool
     */
    public function containsAll($needles, $ignoreCase = false)
    {}

    /**
     * Determine if a given string doesn't contain a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  bool  $ignoreCase
     * @return bool
     */
    public function doesntContain($needles, $ignoreCase = false)
    {}

    /**
     * Convert the case of a string.
     *
     * @param  int  $mode
     * @param  string|null  $encoding
     * @return static
     */
    public function convertCase(int $mode = MB_CASE_FOLD, ?string $encoding = 'UTF-8')
    {}

    /**
     * Get the plural form of an English word with the count prepended.
     *
     * @param  int|array|\Countable  $count
     * @return static
     */
    public function counted($count)
    {}

    /**
     * Replace consecutive instances of a given character with a single character.
     *
     * @param  array<string>|string  $characters
     * @return static
     */
    public function deduplicate(array|string $characters = ' ')
    {}

    /**
     * Get the parent directory's path.
     *
     * @param  int  $levels
     * @return static
     */
    public function dirname($levels = 1)
    {}

    /**
     * Determine if a given string ends with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @return bool
     */
    public function endsWith($needles)
    {}

    /**
     * Determine if a given string doesn't end with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @return bool
     */
    public function doesntEndWith($needles)
    {}

    /**
     * Determine if the string is an exact match with the given value.
     *
     * @param  \Illuminate\Support\Stringable|string  $value
     * @return bool
     */
    public function exactly($value)
    {}

    /**
     * Extracts an excerpt from text that matches the first instance of a phrase.
     *
     * @param  string  $phrase
     * @param  array  $options
     * @return string|null
     */
    public function excerpt($phrase = '', $options = [])
    {}

    /**
     * Explode the string into a collection.
     *
     * @param  string  $delimiter
     * @param  int  $limit
     * @return \Illuminate\Support\Collection<int, string>
     */
    public function explode($delimiter, $limit = PHP_INT_MAX)
    {}

    /**
     * Split a string using a regular expression or by length.
     *
     * @param  string|int  $pattern
     * @param  int  $limit
     * @param  int  $flags
     * @return \Illuminate\Support\Collection<int, string>
     */
    public function split($pattern, $limit = -1, $flags = 0)
    {}

    /**
     * Cap a string with a single instance of a given value.
     *
     * @param  string  $cap
     * @return static
     */
    public function finish($cap)
    {}

    /**
     * Determine if a given string matches a given pattern.
     *
     * @param  string|iterable<string>  $pattern
     * @param  bool  $ignoreCase
     * @return bool
     */
    public function is($pattern, $ignoreCase = false)
    {}

    /**
     * Determine if a given string is 7 bit ASCII.
     *
     * @return bool
     */
    public function isAscii()
    {}

    /**
     * Determine if a given string is valid JSON.
     *
     * @return bool
     */
    public function isJson()
    {}

    /**
     * Determine if a given value is a valid URL.
     *
     * @param  array  $protocols
     * @return bool
     */
    public function isUrl(array $protocols = [])
    {}

    /**
     * Determine if a given string is a valid UUID.
     *
     * @param  int<0, 8>|'max'|null  $version
     * @return bool
     */
    public function isUuid($version = null)
    {}

    /**
     * Determine if a given string is a valid ULID.
     *
     * @return bool
     */
    public function isUlid()
    {}

    /**
     * Determine if the given string is empty.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the given string is not empty.
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Convert a string to kebab case.
     *
     * @return static
     */
    public function kebab()
    {}

    /**
     * Return the length of the given string.
     *
     * @param  string|null  $encoding
     * @return int
     */
    public function length($encoding = null)
    {}

    /**
     * Limit the number of characters in a string.
     *
     * @param  int  $limit
     * @param  string  $end
     * @param  bool  $preserveWords
     * @return static
     */
    public function limit($limit = 100, $end = '...', $preserveWords = false)
    {}

    /**
     * Convert the given string to lower-case.
     *
     * @return static
     */
    public function lower()
    {}

    /**
     * Convert GitHub flavored Markdown into HTML.
     *
     * @param  array  $options
     * @param  array  $extensions
     * @return static
     */
    public function markdown(array $options = [], array $extensions = [])
    {}

    /**
     * Convert inline Markdown into HTML.
     *
     * @param  array  $options
     * @param  array  $extensions
     * @return static
     */
    public function inlineMarkdown(array $options = [], array $extensions = [])
    {}

    /**
     * Masks a portion of a string with a repeated character.
     *
     * @param  string  $character
     * @param  int  $index
     * @param  int|null  $length
     * @param  string  $encoding
     * @return static
     */
    public function mask($character, $index, $length = null, $encoding = 'UTF-8')
    {}

    /**
     * Get the string matching the given pattern.
     *
     * @param  string  $pattern
     * @return static
     */
    public function match($pattern)
    {}

    /**
     * Determine if a given string matches a given pattern.
     *
     * @param  string|iterable<string>  $pattern
     * @return bool
     */
    public function isMatch($pattern)
    {}

    /**
     * Get the string matching the given pattern.
     *
     * @param  string  $pattern
     * @return \Illuminate\Support\Collection
     */
    public function matchAll($pattern)
    {}

    /**
     * Determine if the string matches the given pattern.
     *
     * @param  string  $pattern
     * @return bool
     */
    public function test($pattern)
    {}

    /**
     * Remove all non-numeric characters from a string.
     *
     * @return static
     */
    public function numbers()
    {}

    /**
     * Pad both sides of the string with another.
     *
     * @param  int  $length
     * @param  string  $pad
     * @return static
     */
    public function padBoth($length, $pad = ' ')
    {}

    /**
     * Pad the left side of the string with another.
     *
     * @param  int  $length
     * @param  string  $pad
     * @return static
     */
    public function padLeft($length, $pad = ' ')
    {}

    /**
     * Pad the right side of the string with another.
     *
     * @param  int  $length
     * @param  string  $pad
     * @return static
     */
    public function padRight($length, $pad = ' ')
    {}

    /**
     * Parse a Class@method style callback into class and method.
     *
     * @param  string|null  $default
     * @return array<int, string|null>
     */
    public function parseCallback($default = null)
    {}

    /**
     * Call the given callback and return a new string.
     *
     * @param  callable  $callback
     * @return static
     */
    public function pipe(callable $callback)
    {}

    /**
     * Get the plural form of an English word.
     *
     * @param  int|array|\Countable  $count
     * @param  bool  $prependCount
     * @return static
     */
    public function plural($count = 2, $prependCount = false)
    {}

    /**
     * Pluralize the last word of an English, studly caps case string.
     *
     * @param  int|array|\Countable  $count
     * @return static
     */
    public function pluralStudly($count = 2)
    {}

    /**
     * Pluralize the last word of an English, Pascal caps case string.
     *
     * @param  int|array|\Countable  $count
     * @return static
     */
    public function pluralPascal($count = 2)
    {}

    /**
     * Find the multi-byte safe position of the first occurrence of the given substring.
     *
     * @param  string  $needle
     * @param  int  $offset
     * @param  string|null  $encoding
     * @return int|false
     */
    public function position($needle, $offset = 0, $encoding = null)
    {}

    /**
     * Prepend the given values to the string.
     *
     * @param  string  ...$values
     * @return static
     */
    public function prepend(...$values)
    {}

    /**
     * Remove any occurrence of the given string in the subject.
     *
     * @param  string|iterable<string>  $search
     * @param  bool  $caseSensitive
     * @return static
     */
    public function remove($search, $caseSensitive = true)
    {}

    /**
     * Reverse the string.
     *
     * @return static
     */
    public function reverse()
    {}

    /**
     * Repeat the string.
     *
     * @param  int  $times
     * @return static
     */
    public function repeat(int $times)
    {}

    /**
     * Replace the given value in the given string.
     *
     * @param  string|iterable<string>  $search
     * @param  string|iterable<string>  $replace
     * @param  bool  $caseSensitive
     * @return static
     */
    public function replace($search, $replace, $caseSensitive = true)
    {}

    /**
     * Replace a given value in the string sequentially with an array.
     *
     * @param  string  $search
     * @param  iterable<string>  $replace
     * @return static
     */
    public function replaceArray($search, $replace)
    {}

    /**
     * Replace the first occurrence of a given value in the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @return static
     */
    public function replaceFirst($search, $replace)
    {}

    /**
     * Replace the first occurrence of the given value if it appears at the start of the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @return static
     */
    public function replaceStart($search, $replace)
    {}

    /**
     * Replace the last occurrence of a given value in the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @return static
     */
    public function replaceLast($search, $replace)
    {}

    /**
     * Replace the last occurrence of a given value if it appears at the end of the string.
     *
     * @param  string  $search
     * @param  string  $replace
     * @return static
     */
    public function replaceEnd($search, $replace)
    {}

    /**
     * Replace the patterns matching the given regular expression.
     *
     * @param  array|string  $pattern
     * @param  \Closure|string[]|string  $replace
     * @param  int  $limit
     * @return static
     */
    public function replaceMatches($pattern, $replace, $limit = -1)
    {}

    /**
     * Parse input from a string to a collection, according to a format.
     *
     * @param  string  $format
     * @return \Illuminate\Support\Collection
     */
    public function scan($format)
    {}

    /**
     * Remove all "extra" blank space from the given string.
     *
     * @return static
     */
    public function squish()
    {}

    /**
     * Begin a string with a single instance of a given value.
     *
     * @param  string  $prefix
     * @return static
     */
    public function start($prefix)
    {}

    /**
     * Strip HTML and PHP tags from the given string.
     *
     * @param  string[]|string|null  $allowedTags
     * @return static
     */
    public function stripTags($allowedTags = null)
    {}

    /**
     * Convert the given string to upper-case.
     *
     * @return static
     */
    public function upper()
    {}

    /**
     * Convert the given string to proper case.
     *
     * @return static
     */
    public function title()
    {}

    /**
     * Convert the given string to proper case for each word.
     *
     * @return static
     */
    public function headline()
    {}

    /**
     * Convert the given string to only its initials.
     *
     * @param  bool  $capitalize
     * @return static
     */
    public function initials($capitalize = false)
    {}

    /**
     * Convert the given string to APA-style title case.
     *
     * @return static
     */
    public function apa()
    {}

    /**
     * Transliterate a string to its closest ASCII representation.
     *
     * @param  string|null  $unknown
     * @param  bool|null  $strict
     * @return static
     */
    public function transliterate($unknown = '?', $strict = false)
    {}

    /**
     * Get the singular form of an English word.
     *
     * @return static
     */
    public function singular()
    {}

    /**
     * Generate a URL friendly "slug" from a given string.
     *
     * @param  string  $separator
     * @param  string|null  $language
     * @param  array<string, string>  $dictionary
     * @return static
     */
    public function slug($separator = '-', $language = 'en', $dictionary = ['@' => 'at'])
    {}

    /**
     * Convert a string to snake case.
     *
     * @param  string  $delimiter
     * @return static
     */
    public function snake($delimiter = '_')
    {}

    /**
     * Determine if a given string starts with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @return bool
     */
    public function startsWith($needles)
    {}

    /**
     * Determine if a given string doesn't start with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @return bool
     */
    public function doesntStartWith($needles)
    {}

    /**
     * Convert a value to studly caps case.
     *
     * @param  bool  $normalize
     * @return static
     */
    public function studly(bool $normalize = false)
    {}

    /**
     * Convert the string to Pascal case.
     *
     * @param  bool  $normalize
     * @return static
     */
    public function pascal(bool $normalize = false)
    {}

    /**
     * Returns the portion of the string specified by the start and length parameters.
     *
     * @param  int  $start
     * @param  int|null  $length
     * @param  string  $encoding
     * @return static
     */
    public function substr($start, $length = null, $encoding = 'UTF-8')
    {}

    /**
     * Returns the number of substring occurrences.
     *
     * @param  string  $needle
     * @param  int  $offset
     * @param  int|null  $length
     * @return int
     */
    public function substrCount($needle, $offset = 0, $length = null)
    {}

    /**
     * Replace text within a portion of a string.
     *
     * @param  string|string[]  $replace
     * @param  int|int[]  $offset
     * @param  int|int[]|null  $length
     * @return static
     */
    public function substrReplace($replace, $offset = 0, $length = null)
    {}

    /**
     * Swap multiple keywords in a string with other keywords.
     *
     * @param  array  $map
     * @return static
     */
    public function swap(array $map)
    {}

    /**
     * Take the first or last {$limit} characters.
     *
     * @param  int  $limit
     * @return static
     */
    public function take(int $limit)
    {}

    /**
     * Trim the string of the given characters.
     *
     * @param  string|null  $characters
     * @return static
     */
    public function trim($characters = null)
    {}

    /**
     * Left trim the string of the given characters.
     *
     * @param  string|null  $characters
     * @return static
     */
    public function ltrim($characters = null)
    {}

    /**
     * Right trim the string of the given characters.
     *
     * @param  string|null  $characters
     * @return static
     */
    public function rtrim($characters = null)
    {}

    /**
     * Make a string's first character lowercase.
     *
     * @return static
     */
    public function lcfirst()
    {}

    /**
     * Make a string's first character uppercase.
     *
     * @return static
     */
    public function ucfirst()
    {}

    /**
     * Capitalize the first character of each word in a string.
     *
     * @param  string  $separators
     * @return static
     */
    public function ucwords($separators = " \t\r\n\f\v")
    {}

    /**
     * Split a string by uppercase characters.
     *
     * @return \Illuminate\Support\Collection<int, string>
     */
    public function ucsplit()
    {}

    /**
     * Execute the given callback if the string contains a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenContains($needles, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string contains all array values.
     *
     * @param  iterable<string>  $needles
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenContainsAll(array $needles, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is empty.
     *
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenEmpty($callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is not empty.
     *
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenNotEmpty($callback, $default = null)
    {}

    /**
     * Execute the given callback if the string ends with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenEndsWith($needles, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string doesn't end with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenDoesntEndWith($needles, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is an exact match with the given value.
     *
     * @param  string  $value
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenExactly($value, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is not an exact match with the given value.
     *
     * @param  string  $value
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenNotExactly($value, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string matches a given pattern.
     *
     * @param  string|iterable<string>  $pattern
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenIs($pattern, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is 7 bit ASCII.
     *
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenIsAscii($callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is a valid UUID.
     *
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenIsUuid($callback, $default = null)
    {}

    /**
     * Execute the given callback if the string is a valid ULID.
     *
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenIsUlid($callback, $default = null)
    {}

    /**
     * Execute the given callback if the string starts with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenStartsWith($needles, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string doesn't start with a given substring.
     *
     * @param  string|iterable<string>  $needles
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenDoesntStartWith($needles, $callback, $default = null)
    {}

    /**
     * Execute the given callback if the string matches the given pattern.
     *
     * @param  string  $pattern
     * @param  callable  $callback
     * @param  callable|null  $default
     * @return static
     */
    public function whenTest($pattern, $callback, $default = null)
    {}

    /**
     * Limit the number of words in a string.
     *
     * @param  int  $words
     * @param  string  $end
     * @return static
     */
    public function words($words = 100, $end = '...')
    {}

    /**
     * Get the number of words a string contains.
     *
     * @param  string|null  $characters
     * @return int
     */
    public function wordCount($characters = null)
    {}

    /**
     * Wrap a string to a given number of characters.
     *
     * @param  int  $characters
     * @param  string  $break
     * @param  bool  $cutLongWords
     * @return static
     */
    public function wordWrap($characters = 75, $break = "\n", $cutLongWords = false)
    {}

    /**
     * Wrap the string with the given strings.
     *
     * @param  string  $before
     * @param  string|null  $after
     * @return static
     */
    public function wrap($before, $after = null)
    {}

    /**
     * Unwrap the string with the given strings.
     *
     * @param  string  $before
     * @param  string|null  $after
     * @return static
     */
    public function unwrap($before, $after = null)
    {}

    /**
     * Convert the string into a `HtmlString` instance.
     *
     * @return \Illuminate\Support\HtmlString
     */
    public function toHtmlString()
    {}

    /**
     * Convert the string to Base64 encoding.
     *
     * @return static
     */
    public function toBase64()
    {}

    /**
     * Decode the Base64 encoded string.
     *
     * @param  bool  $strict
     * @return static
     */
    public function fromBase64($strict = false)
    {}

    /**
     * Hash the string using the given algorithm.
     *
     * @param  string  $algorithm
     * @return static
     */
    public function hash(string $algorithm)
    {}

    /**
     * Encrypt the string.
     *
     * @param  bool  $serialize
     * @return static
     */
    public function encrypt(bool $serialize = false)
    {}

    /**
     * Decrypt the string.
     *
     * @param  bool  $serialize
     * @return static
     */
    public function decrypt(bool $serialize = false)
    {}

    /**
     * Dump the string.
     *
     * @param  mixed  ...$args
     * @return $this
     */
    public function dump(...$args)
    {}

    /**
     * Get the underlying string value.
     *
     * @return string
     */
    public function value()
    {}

    /**
     * Get the underlying string value.
     *
     * @return string
     */
    public function toString()
    {}

    /**
     * Get the underlying string value as an integer.
     *
     * @param  int  $base
     * @return int
     */
    public function toInteger($base = 10)
    {}

    /**
     * Get the underlying string value as a float.
     *
     * @return float
     */
    public function toFloat()
    {}

    /**
     * Get the underlying string value as a boolean.
     *
     * Returns true when value is "1", "true", "on", and "yes". Otherwise, returns false.
     *
     * @return bool
     */
    public function toBoolean()
    {}

    /**
     * Get the underlying string value as a Carbon instance.
     *
     * @param  string|null  $format
     * @param  string|null  $tz
     * @return \Illuminate\Support\Carbon
     *
     * @throws \Carbon\Exceptions\InvalidFormatException
     */
    public function toDate($format = null, $tz = null)
    {}

    /**
     * Get the underlying string value as a Uri instance.
     *
     * @return \Illuminate\Support\Uri
     */
    public function toUri()
    {}

    /**
     * Convert the object to a string when JSON encoded.
     *
     * @return string
     */
    public function jsonSerialize(): string
    {}

    /**
     * Determine if the given offset exists.
     *
     * @param  mixed  $offset
     * @return bool
     */
    public function offsetExists(mixed $offset): bool
    {}

    /**
     * Get the value at the given offset.
     *
     * @param  mixed  $offset
     * @return string
     */
    public function offsetGet(mixed $offset): string
    {}

    /**
     * Set the value at the given offset.
     *
     * @param  mixed  $offset
     * @return void
     */
    public function offsetSet(mixed $offset, mixed $value): void
    {}

    /**
     * Unset the value at the given offset.
     *
     * @param  mixed  $offset
     * @return void
     */
    public function offsetUnset(mixed $offset): void
    {}

    /**
     * Proxy dynamic properties onto methods.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Get the raw string value.
     *
     * @return string
     */
    public function __toString()
    {}
}
