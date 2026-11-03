<?php

namespace Illuminate\Support;

use Illuminate\Support\Traits\Macroable;
use NumberFormatter;
use RuntimeException;

class Number
{
    use Macroable;

    /**
     * The current default locale.
     *
     * @var string
     */
    protected static $locale = 'en';

    /**
     * The current default currency.
     *
     * @var string
     */
    protected static $currency = 'USD';

    /**
     * Format the given number according to the current locale.
     *
     * @param  int|float  $number
     * @param  int|null  $precision
     * @param  int|null  $maxPrecision
     * @param  string|null  $locale
     * @return string|false
     */
    public static function format(int|float $number, ?int $precision = null, ?int $maxPrecision = null, ?string $locale = null)
    {}

    /**
     * Parse the given string according to the specified format type.
     *
     * @param  string  $string
     * @param  int|null  $type
     * @param  string|null  $locale
     * @return int|float|false
     */
    public static function parse(string $string, ?int $type = NumberFormatter::TYPE_DOUBLE, ?string $locale = null): int|float|false
    {}

    /**
     * Parse a string into an integer according to the specified locale.
     *
     * @param  string  $string
     * @param  string|null  $locale
     * @return int|false
     */
    public static function parseInt(string $string, ?string $locale = null): int|false
    {}

    /**
     * Parse a string into a float according to the specified locale.
     *
     * @param  string  $string
     * @param  string|null  $locale
     * @return float|false
     */
    public static function parseFloat(string $string, ?string $locale = null): float|false
    {}

    /**
     * Spell out the given number in the given locale.
     *
     * @param  int|float  $number
     * @param  string|null  $locale
     * @param  int|null  $after
     * @param  int|null  $until
     * @return string|false
     */
    public static function spell(int|float $number, ?string $locale = null, ?int $after = null, ?int $until = null)
    {}

    /**
     * Convert the given number to ordinal form.
     *
     * @param  int|float  $number
     * @param  string|null  $locale
     * @return string|false
     */
    public static function ordinal(int|float $number, ?string $locale = null)
    {}

    /**
     * Spell out the given number in the given locale in ordinal form.
     *
     * @param  int|float  $number
     * @param  string|null  $locale
     * @return string|false
     */
    public static function spellOrdinal(int|float $number, ?string $locale = null)
    {}

    /**
     * Convert the given number to its percentage equivalent.
     *
     * @param  int|float  $number
     * @param  int  $precision
     * @param  int|null  $maxPrecision
     * @param  string|null  $locale
     * @return string|false
     */
    public static function percentage(int|float $number, int $precision = 0, ?int $maxPrecision = null, ?string $locale = null)
    {}

    /**
     * Convert the given number to its currency equivalent.
     *
     * @param  int|float  $number
     * @param  string  $in
     * @param  string|null  $locale
     * @param  int|null  $precision
     * @return string|false
     */
    public static function currency(int|float $number, string $in = '', ?string $locale = null, ?int $precision = null)
    {}

    /**
     * Convert the given number to its file size equivalent.
     *
     * @param  int|float  $bytes
     * @param  int  $precision
     * @param  int|null  $maxPrecision
     * @return string
     */
    public static function fileSize(int|float $bytes, int $precision = 0, ?int $maxPrecision = null)
    {}

    /**
     * Convert the number to its human-readable equivalent.
     *
     * @param  int|float  $number
     * @param  int  $precision
     * @param  int|null  $maxPrecision
     * @return string|false
     */
    public static function abbreviate(int|float $number, int $precision = 0, ?int $maxPrecision = null)
    {}

    /**
     * Convert the number to its human-readable equivalent.
     *
     * @param  int|float  $number
     * @param  int  $precision
     * @param  int|null  $maxPrecision
     * @param  bool  $abbreviate
     * @return string|false
     */
    public static function forHumans(int|float $number, int $precision = 0, ?int $maxPrecision = null, bool $abbreviate = false)
    {}

    /**
     * Convert the number to its human-readable equivalent.
     *
     * @param  int|float  $number
     * @param  int  $precision
     * @param  int|null  $maxPrecision
     * @param  array<int, string>  $units
     * @return string|false
     *
     * @phpstan-return ($number is INF ? '∞' : ($number is NAN ? 'NaN' : ($number is 0 ? ($precision is non-positive-int ? '0' : non-empty-string|false) : non-empty-string|false)))
     */
    protected static function summarize(int|float $number, int $precision = 0, ?int $maxPrecision = null, array $units = [])
    {}

    /**
     * Clamp the given number between the given minimum and maximum.
     *
     * @param  int|float  $number
     * @param  int|float  $min
     * @param  int|float  $max
     * @return int|float
     */
    public static function clamp(int|float $number, int|float $min, int|float $max)
    {}

    /**
     * Split the given number into pairs of min/max values.
     *
     * @param  int|float  $to
     * @param  int|float  $by
     * @param  int|float  $start
     * @param  int|float  $offset
     * @return list<array{int|float, int|float}>
     */
    public static function pairs(int|float $to, int|float $by, int|float $start = 0, int|float $offset = 1)
    {}

    /**
     * Remove any trailing zero digits after the decimal point of the given number.
     *
     * @param  int|float  $number
     * @return int|float
     */
    public static function trim(int|float $number)
    {}

    /**
     * Execute the given callback using the given locale.
     *
     * @template TReturn
     *
     * @param  string  $locale
     * @param  callable(): TReturn  $callback
     * @return TReturn
     */
    public static function withLocale(string $locale, callable $callback)
    {}

    /**
     * Execute the given callback using the given currency.
     *
     * @template TReturn
     *
     * @param  string  $currency
     * @param  callable(): TReturn  $callback
     * @return TReturn
     */
    public static function withCurrency(string $currency, callable $callback)
    {}

    /**
     * Set the default locale.
     *
     * @param  string  $locale
     * @return void
     */
    public static function useLocale(string $locale)
    {}

    /**
     * Set the default currency.
     *
     * @param  string  $currency
     * @return void
     */
    public static function useCurrency(string $currency)
    {}

    /**
     * Get the default locale.
     *
     * @return string
     */
    public static function defaultLocale()
    {}

    /**
     * Get the default currency.
     *
     * @return string
     */
    public static function defaultCurrency()
    {}

    /**
     * Ensure the "intl" PHP extension is installed.
     *
     * @return void
     *
     * @throws \RuntimeException
     */
    protected static function ensureIntlExtensionIsInstalled()
    {}

    /**
     * Replace a negative number that would be formatted as zero with zero, avoiding a "-0" result.
     *
     * @param  int|float  $number
     * @param  callable(int|float): (string|false)  $format
     * @return int|float
     */
    protected static function withoutNegativeZero(int|float $number, callable $format)
    {}
}
