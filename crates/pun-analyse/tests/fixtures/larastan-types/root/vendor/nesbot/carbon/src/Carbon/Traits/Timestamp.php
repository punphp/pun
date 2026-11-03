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

use DateTimeZone;
use ReturnTypeWillChange;

/**
 * Trait Timestamp.
 */
trait Timestamp
{
    /**
     * Create a Carbon instance from a timestamp and set the timezone (UTC by default).
     *
     * Timestamp input can be given as int, float or a string containing one or more numbers.
     */
    #[ReturnTypeWillChange]
    public static function createFromTimestamp(
        float|int|string $timestamp,
        DateTimeZone|string|int|null $timezone = null,
    ): static {}

    /**
     * Create a Carbon instance from a timestamp keeping the timezone to UTC.
     *
     * Timestamp input can be given as int, float or a string containing one or more numbers.
     */
    public static function createFromTimestampUTC(float|int|string $timestamp): static
    {}

    /**
     * Create a Carbon instance from a timestamp in milliseconds.
     *
     * Timestamp input can be given as int, float or a string containing one or more numbers.
     *
     * @param float|int|string $timestamp
     *
     * @return static
     */
    public static function createFromTimestampMsUTC($timestamp): static
    {}

    /**
     * Create a Carbon instance from a timestamp in milliseconds.
     *
     * Timestamp input can be given as int, float or a string containing one or more numbers.
     */
    public static function createFromTimestampMs(
        float|int|string $timestamp,
        DateTimeZone|string|int|null $timezone = null,
    ): static {}

    /**
     * Set the instance's timestamp.
     *
     * Timestamp input can be given as int, float or a string containing one or more numbers.
     */
    public function timestamp(float|int|string $timestamp): static
    {}

    /**
     * Returns a timestamp rounded with the given precision (6 by default).
     *
     * @example getPreciseTimestamp()   1532087464437474 (microsecond maximum precision)
     * @example getPreciseTimestamp(6)  1532087464437474
     * @example getPreciseTimestamp(5)  153208746443747  (1/100000 second precision)
     * @example getPreciseTimestamp(4)  15320874644375   (1/10000 second precision)
     * @example getPreciseTimestamp(3)  1532087464437    (millisecond precision)
     * @example getPreciseTimestamp(2)  153208746444     (1/100 second precision)
     * @example getPreciseTimestamp(1)  15320874644      (1/10 second precision)
     * @example getPreciseTimestamp(0)  1532087464       (second precision)
     * @example getPreciseTimestamp(-1) 153208746        (10 second precision)
     * @example getPreciseTimestamp(-2) 15320875         (100 second precision)
     *
     * @param int $precision
     *
     * @return float
     */
    public function getPreciseTimestamp($precision = 6): float
    {}

    /**
     * Returns the milliseconds timestamps used amongst other by Date javascript objects.
     *
     * @return float
     */
    public function valueOf(): float
    {}

    /**
     * Returns the timestamp with millisecond precision.
     *
     * @return int
     */
    public function getTimestampMs(): int
    {}

    /**
     * @alias getTimestamp
     *
     * Returns the UNIX timestamp for the current date.
     *
     * @return int
     */
    public function unix(): int
    {}

    /**
     * Return an array with integer part digits and decimals digits split from one or more positive numbers
     * (such as timestamps) as string with the given number of decimals (6 by default).
     *
     * By splitting integer and decimal, this method obtain a better precision than
     * number_format when the input is a string.
     *
     * @param float|int|string $numbers  one or more numbers
     * @param int              $decimals number of decimals precision (6 by default)
     *
     * @return array 0-index is integer part, 1-index is decimal part digits
     */
    private static function getIntegerAndDecimalParts($numbers, $decimals = 6): array
    {}
}
