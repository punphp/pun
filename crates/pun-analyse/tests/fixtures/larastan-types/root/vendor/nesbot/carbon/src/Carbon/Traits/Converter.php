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
use Carbon\CarbonInterval;
use Carbon\CarbonPeriod;
use Carbon\CarbonPeriodImmutable;
use Carbon\Exceptions\UnitException;
use Closure;
use DateTime;
use DateTimeImmutable;
use DateTimeInterface;
use Time\Duration;

/**
 * Trait Converter.
 *
 * Change date into different string formats and types and
 * handle the string cast.
 *
 * Depends on the following methods:
 *
 * @method static copy()
 */
trait Converter
{
    use ToStringFormat;

    /**
     * Returns the formatted date string on success or FALSE on failure.
     *
     * @see https://php.net/manual/en/datetime.format.php
     */
    public function format(string $format): string
    { func_get_args(); }

    /**
     * @see https://php.net/manual/en/datetime.format.php
     */
    public function rawFormat(string $format): string
    {}

    /**
     * Format the instance as a string using the set format
     *
     * @example
     * ```
     * echo Carbon::now(); // Carbon instances can be cast to string
     * ```
     */
    public function __toString(): string
    {}

    /**
     * Format the instance as date
     *
     * @example
     * ```
     * echo Carbon::now()->toDateString();
     * ```
     */
    public function toDateString(): string
    {}

    /**
     * Format the instance as a readable date
     *
     * @example
     * ```
     * echo Carbon::now()->toFormattedDateString();
     * ```
     */
    public function toFormattedDateString(): string
    {}

    /**
     * Format the instance with the day, and a readable date
     *
     * @example
     * ```
     * echo Carbon::now()->toFormattedDayDateString();
     * ```
     */
    public function toFormattedDayDateString(): string
    {}

    /**
     * Format the instance as time
     *
     * @example
     * ```
     * echo Carbon::now()->toTimeString();
     * ```
     */
    public function toTimeString(string $unitPrecision = 'second'): string
    {}

    /**
     * Format the instance as date and time
     *
     * @example
     * ```
     * echo Carbon::now()->toDateTimeString();
     * ```
     */
    public function toDateTimeString(string $unitPrecision = 'second'): string
    {}

    /**
     * Return a format from H:i to H:i:s.u according to given unit precision.
     *
     * @param string $unitPrecision "minute", "second", "millisecond" or "microsecond"
     */
    public static function getTimeFormatByPrecision(string $unitPrecision): string
    {}

    /**
     * Format the instance as date and time T-separated with no timezone
     *
     * @example
     * ```
     * echo Carbon::now()->toDateTimeLocalString();
     * echo "\n";
     * echo Carbon::now()->toDateTimeLocalString('minute'); // You can specify precision among: minute, second, millisecond and microsecond
     * ```
     */
    public function toDateTimeLocalString(string $unitPrecision = 'second'): string
    {}

    /**
     * Format the instance with day, date and time
     *
     * @example
     * ```
     * echo Carbon::now()->toDayDateTimeString();
     * ```
     */
    public function toDayDateTimeString(): string
    {}

    /**
     * Format the instance as ATOM
     *
     * @example
     * ```
     * echo Carbon::now()->toAtomString();
     * ```
     */
    public function toAtomString(): string
    {}

    /**
     * Format the instance as COOKIE
     *
     * @example
     * ```
     * echo Carbon::now()->toCookieString();
     * ```
     */
    public function toCookieString(): string
    {}

    /**
     * Format the instance as ISO8601
     *
     * @example
     * ```
     * echo Carbon::now()->toIso8601String();
     * ```
     */
    public function toIso8601String(): string
    {}

    /**
     * Format the instance as RFC822
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc822String();
     * ```
     */
    public function toRfc822String(): string
    {}

    /**
     * Convert the instance to UTC and return as Zulu ISO8601
     *
     * @example
     * ```
     * echo Carbon::now()->toIso8601ZuluString();
     * ```
     */
    public function toIso8601ZuluString(string $unitPrecision = 'second'): string
    {}

    /**
     * Format the instance as RFC850
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc850String();
     * ```
     */
    public function toRfc850String(): string
    {}

    /**
     * Format the instance as RFC1036
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc1036String();
     * ```
     */
    public function toRfc1036String(): string
    {}

    /**
     * Format the instance as RFC1123
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc1123String();
     * ```
     */
    public function toRfc1123String(): string
    {}

    /**
     * Format the instance as RFC2822
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc2822String();
     * ```
     */
    public function toRfc2822String(): string
    {}

    /**
     * Format the instance as RFC3339.
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc3339String() . "\n";
     * echo Carbon::now()->toRfc3339String(true) . "\n";
     * ```
     */
    public function toRfc3339String(bool $extended = false): string
    {}

    /**
     * Format the instance as RSS
     *
     * @example
     * ```
     * echo Carbon::now()->toRssString();
     * ```
     */
    public function toRssString(): string
    {}

    /**
     * Format the instance as W3C
     *
     * @example
     * ```
     * echo Carbon::now()->toW3cString();
     * ```
     */
    public function toW3cString(): string
    {}

    /**
     * Format the instance as RFC7231
     *
     * @example
     * ```
     * echo Carbon::now()->toRfc7231String();
     * ```
     */
    public function toRfc7231String(): string
    {}

    /**
     * Get default array representation.
     *
     * @example
     * ```
     * var_dump(Carbon::now()->toArray());
     * ```
     */
    public function toArray(): array
    {}

    /**
     * Get default object representation.
     *
     * @example
     * ```
     * var_dump(Carbon::now()->toObject());
     * ```
     */
    public function toObject(): object
    {}

    /**
     * Returns english human-readable complete date string.
     *
     * @example
     * ```
     * echo Carbon::now()->toString();
     * ```
     */
    public function toString(): string
    {}

    /**
     * Return the ISO-8601 string (ex: 1977-04-22T06:00:00Z, if $keepOffset truthy, offset will be kept:
     * 1977-04-22T01:00:00-05:00).
     *
     * @example
     * ```
     * echo Carbon::now('America/Toronto')->toISOString() . "\n";
     * echo Carbon::now('America/Toronto')->toISOString(true) . "\n";
     * ```
     *
     * @param bool $keepOffset Pass true to keep the date offset. Else forced to UTC.
     */
    public function toISOString(bool $keepOffset = false): ?string
    {}

    /**
     * Return the ISO-8601 string (ex: 1977-04-22T06:00:00Z) with UTC timezone.
     *
     * @example
     * ```
     * echo Carbon::now('America/Toronto')->toJSON();
     * ```
     */
    public function toJSON(): ?string
    {}

    /**
     * Return native DateTime PHP object matching the current instance.
     *
     * @example
     * ```
     * var_dump(Carbon::now()->toDateTime());
     * ```
     */
    public function toDateTime(): DateTime
    {}

    /**
     * Return native toDateTimeImmutable PHP object matching the current instance.
     *
     * @example
     * ```
     * var_dump(Carbon::now()->toDateTimeImmutable());
     * ```
     */
    public function toDateTimeImmutable(): DateTimeImmutable
    {}

    /**
     * @alias toDateTime
     *
     * Return native DateTime PHP object matching the current instance.
     *
     * @example
     * ```
     * var_dump(Carbon::now()->toDate());
     * ```
     */
    public function toDate(): DateTime
    {}

    /**
     * Create a iterable CarbonPeriod object from current date to a given end date (and optional interval).
     *
     * @param \DateTimeInterface|Carbon|CarbonImmutable|int|null $end      period end date or recurrences count if int
     * @param int|\DateInterval|Duration|string|null             $interval period default interval or number of the given $unit
     * @param string|null                                        $unit     if specified, $interval must be an integer
     */
    public function toPeriod($end = null, $interval = null, $unit = null): CarbonPeriod
    {}

    /**
     * Create a iterable CarbonPeriod object from current date to a given end date (and optional interval).
     *
     * @param \DateTimeInterface|Carbon|CarbonImmutable|null $end      period end date
     * @param int|\DateInterval|Duration|string|null         $interval period default interval or number of the given $unit
     * @param string|null                                    $unit     if specified, $interval must be an integer
     */
    public function range($end = null, $interval = null, $unit = null): CarbonPeriod
    {}
}
