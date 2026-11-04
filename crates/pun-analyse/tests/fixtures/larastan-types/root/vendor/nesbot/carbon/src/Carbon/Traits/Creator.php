<?php

declare(strict_types=1);

/**
 * This file is part of the Carbon package.
 *
 * (c) Brian Nesbitt <brian@nesbot.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Carbon\Traits;

use Carbon\Carbon;
use Carbon\CarbonImmutable;
use Carbon\CarbonInterface;
use Carbon\Exceptions\InvalidDateException;
use Carbon\Exceptions\InvalidFormatException;
use Carbon\Exceptions\InvalidTimeZoneException;
use Carbon\Exceptions\OutOfRangeException;
use Carbon\Exceptions\UnitException;
use Carbon\Month;
use Carbon\Translator;
use Carbon\WeekDay;
use Closure;
use DateMalformedStringException;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Exception;
use ReturnTypeWillChange;
use Symfony\Contracts\Translation\TranslatorInterface;

/**
 * Trait Creator.
 *
 * Static creators.
 *
 * Depends on the following methods:
 *
 * @method static Carbon|CarbonImmutable getTestNow()
 */
trait Creator
{
    use ObjectInitialisation;
    use LocalFactory;

    /**
     * The errors that can occur.
     */
    protected static array|bool $lastErrors = false;

    /**
     * Create a new Carbon instance.
     *
     * Please see the testing aids section (specifically static::setTestNow())
     * for more on the possibility of this constructor returning a test instance.
     *
     * @throws InvalidFormatException
     */
    public function __construct(
        DateTimeInterface|WeekDay|Month|string|int|float|null $time = null,
        DateTimeZone|string|int|null $timezone = null,
    ) {}

    /**
     * Get timezone from a datetime instance.
     */
    private function constructTimezoneFromDateTime(
        DateTimeInterface $date,
        DateTimeZone|string|int|null &$timezone,
    ): DateTimeInterface {}

    /**
     * Update constructedObjectId on cloned.
     */
    public function __clone(): void
    {}

    /**
     * Create a Carbon instance from a DateTime one.
     */
    public static function instance(DateTimeInterface $date): static
    {}

    /**
     * Create a carbon instance from a string.
     *
     * This is an alias for the constructor that allows better fluent syntax
     * as it allows you to do Carbon::parse('Monday next week')->fn() rather
     * than (new Carbon('Monday next week'))->fn().
     *
     * @throws InvalidFormatException
     */
    public static function rawParse(
        DateTimeInterface|WeekDay|Month|string|int|float|null $time,
        DateTimeZone|string|int|null $timezone = null,
    ): static {}

    /**
     * Create a carbon instance from a string.
     *
     * This is an alias for the constructor that allows better fluent syntax
     * as it allows you to do Carbon::parse('Monday next week')->fn() rather
     * than (new Carbon('Monday next week'))->fn().
     *
     * @throws InvalidFormatException
     */
    public static function parse(
        DateTimeInterface|WeekDay|Month|string|int|float|null $time,
        DateTimeZone|string|int|null $timezone = null,
    ): static { func_get_args(); }

    /**
     * Create a carbon instance from a localized string (in French, Japanese, Arabic, etc.).
     *
     * @param string                       $time     date/time string in the given language (may also contain English).
     * @param string|null                  $locale   if locale is null or not specified, current global locale will be
     *                                               used instead.
     * @param DateTimeZone|string|int|null $timezone optional timezone for the new instance.
     *
     * @throws InvalidFormatException
     */
    public static function parseFromLocale(
        string $time,
        ?string $locale = null,
        DateTimeZone|string|int|null $timezone = null,
    ): static {}

    /**
     * Get a Carbon instance for the current date and time.
     */
    public static function now(DateTimeZone|string|int|null $timezone = null): static
    {}

    /**
     * Create a Carbon instance for today.
     */
    public static function today(DateTimeZone|string|int|null $timezone = null): static
    {}

    /**
     * Create a Carbon instance for tomorrow.
     */
    public static function tomorrow(DateTimeZone|string|int|null $timezone = null): static
    {}

    /**
     * Create a Carbon instance for yesterday.
     */
    public static function yesterday(DateTimeZone|string|int|null $timezone = null): static
    {}

    private static function assertBetween($unit, $value, $min, $max): void
    {}

    private static function createNowInstance($timezone)
    {}

    /**
     * Create a new Carbon instance from a specific date and time.
     *
     * If any of $year, $month or $day are set to null their now() values will
     * be used.
     *
     * If $hour is null it will be set to its now() value and the default
     * values for $minute and $second will be their now() values.
     *
     * If $hour is not null then the default values for $minute and $second
     * will be 0.
     *
     * @param DateTimeInterface|string|int|null $year
     * @param int|null                          $month
     * @param int|null                          $day
     * @param int|null                          $hour
     * @param int|null                          $minute
     * @param int|null                          $second
     * @param DateTimeZone|string|int|null      $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    public static function create($year = 0, $month = 1, $day = 1, $hour = 0, $minute = 0, $second = 0, $timezone = null): ?static
    {}

    /**
     * Create a new safe Carbon instance from a specific date and time.
     *
     * If any of $year, $month or $day are set to null their now() values will
     * be used.
     *
     * If $hour is null it will be set to its now() value and the default
     * values for $minute and $second will be their now() values.
     *
     * If $hour is not null then the default values for $minute and $second
     * will be 0.
     *
     * If one of the set values is not valid, an InvalidDateException
     * will be thrown.
     *
     * @param int|null                     $year
     * @param int|null                     $month
     * @param int|null                     $day
     * @param int|null                     $hour
     * @param int|null                     $minute
     * @param int|null                     $second
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidDateException
     *
     * @return static|null
     */
    public static function createSafe($year = null, $month = null, $day = null, $hour = null, $minute = null, $second = null, $timezone = null): ?static
    {}

    /**
     * Create a new Carbon instance from a specific date and time using strict validation.
     *
     * @see create()
     *
     * @param int|null                     $year
     * @param int|null                     $month
     * @param int|null                     $day
     * @param int|null                     $hour
     * @param int|null                     $minute
     * @param int|null                     $second
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static
     */
    public static function createStrict(?int $year = 0, ?int $month = 1, ?int $day = 1, ?int $hour = 0, ?int $minute = 0, ?int $second = 0, $timezone = null): static
    {}

    /**
     * Create a Carbon instance from just a date. The time portion is set to now.
     *
     * @param int|null                     $year
     * @param int|null                     $month
     * @param int|null                     $day
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static
     */
    public static function createFromDate($year = null, $month = null, $day = null, $timezone = null)
    {}

    /**
     * Create a Carbon instance from just a date. The time portion is set to midnight.
     *
     * @param int|null                     $year
     * @param int|null                     $month
     * @param int|null                     $day
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static
     */
    public static function createMidnightDate($year = null, $month = null, $day = null, $timezone = null)
    {}

    /**
     * Create a Carbon instance from just a time. The date portion is set to today.
     *
     * @param int|null                     $hour
     * @param int|null                     $minute
     * @param int|null                     $second
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static
     */
    public static function createFromTime($hour = 0, $minute = 0, $second = 0, $timezone = null): static
    {}

    /**
     * Create a Carbon instance from a time string. The date portion is set to today.
     *
     * @throws InvalidFormatException
     */
    public static function createFromTimeString(string $time, DateTimeZone|string|int|null $timezone = null): static
    {}

    private static function createFromFormatAndTimezone(
        string $format,
        string $time,
        DateTimeZone|string|int|null $originalTimezone,
    ): ?DateTimeInterface {}

    private static function getOffsetTimezone(int $offset): string
    {}

    /**
     * Create a Carbon instance from a specific format.
     *
     * @param string                       $format   Datetime format
     * @param string                       $time
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    public static function rawCreateFromFormat(string $format, string $time, $timezone = null): ?static
    {}

    /**
     * Create a Carbon instance from a specific format.
     *
     * @param string                       $format   Datetime format
     * @param string                       $time
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    #[ReturnTypeWillChange]
    public static function createFromFormat($format, $time, $timezone = null): ?static
    { func_get_args(); }

    /**
     * Create a Carbon instance from a specific ISO format (same replacements as ->isoFormat()).
     *
     * Era tokens (N to NNNNN, y, yy, yyy, yyyy, yo) are read with the "eras" of $locale: the era name gives the
     * era and the year is the year of this era, so 'D MMMM N y' with 'th' reads '29 กุมภาพันธ์ พ.ศ. 2567' as
     * 2024-02-29. Without any N token, the era of today's date in this locale is used.
     *
     * @param string                       $format     Datetime format
     * @param string                       $time
     * @param DateTimeZone|string|int|null $timezone   optional timezone
     * @param string|null                  $locale     locale to be used for LTS, LT, LL, LLL, etc. macro-formats (en by fault, unneeded if no such macro-format in use)
     * @param TranslatorInterface|null     $translator optional custom translator to use for macro-formats
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    public static function createFromIsoFormat(
        string $format,
        string $time,
        $timezone = null,
        ?string $locale = CarbonInterface::DEFAULT_LOCALE,
        ?TranslatorInterface $translator = null
    ): ?static {}

    /**
     * Create a Carbon instance from a specific format and a string in a given language.
     *
     * @param string                       $format   Datetime format
     * @param string                       $locale
     * @param string                       $time
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    public static function createFromLocaleFormat(string $format, string $locale, string $time, $timezone = null): ?static
    {}

    /**
     * Create a Carbon instance from a specific ISO format and a string in a given language.
     *
     * @param string                       $format   Datetime ISO format
     * @param string                       $locale
     * @param string                       $time
     * @param DateTimeZone|string|int|null $timezone
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    public static function createFromLocaleIsoFormat(string $format, string $locale, string $time, $timezone = null): ?static
    {}

    /**
     * Make a Carbon instance from given variable if possible.
     *
     * Always return a new instance. Parse only strings and only these likely to be dates (skip intervals
     * and recurrences). Throw an exception for invalid format, but otherwise return null.
     *
     * @param mixed $var
     *
     * @throws InvalidFormatException
     *
     * @return static|null
     */
    public static function make($var, DateTimeZone|string|null $timezone = null): ?static
    {}

    /**
     * Set last errors.
     *
     * @param array|bool $lastErrors
     *
     * @return void
     */
    private static function setLastErrors($lastErrors): void
    {}

    /**
     * {@inheritdoc}
     */
    public static function getLastErrors(): array|false
    {}

    private static function monthToInt(mixed $value, string $unit = 'month'): mixed
    {}
}
