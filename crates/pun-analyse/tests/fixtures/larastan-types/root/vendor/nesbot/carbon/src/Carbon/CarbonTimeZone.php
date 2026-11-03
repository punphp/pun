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

namespace Carbon;

use Carbon\Constants\UnitValue;
use Carbon\Exceptions\InvalidCastException;
use Carbon\Exceptions\InvalidTimeZoneException;
use Carbon\Traits\LocalFactory;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Exception;
use Throwable;

class CarbonTimeZone extends DateTimeZone
{
    use LocalFactory;

    public const MAXIMUM_TIMEZONE_OFFSET = 99;

    public function __construct(string|int|float $timezone)
    {}

    protected static function parseNumericTimezone(string|int|float $timezone): string
    {}

    protected static function getDateTimeZoneNameFromMixed(string|int|float $timezone): string
    {}

    /**
     * Cast the current instance into the given class.
     *
     * @param class-string<DateTimeZone> $className The $className::instance() method will be called to cast the current object.
     *
     * @return DateTimeZone|mixed
     */
    public function cast(string $className): mixed
    {}

    /**
     * Create a CarbonTimeZone from mixed input.
     *
     * @param DateTimeZone|string|int|false|null $object     original value to get CarbonTimeZone from it.
     * @param DateTimeZone|string|int|false|null $objectDump dump of the object for error messages.
     *
     * @throws InvalidTimeZoneException
     *
     * @return static|null
     */
    public static function instance(
        DateTimeZone|string|int|false|null $object,
        DateTimeZone|string|int|false|null $objectDump = null,
    ): ?self {}

    /**
     * Returns abbreviated name of the current timezone according to DST setting.
     *
     * @param bool $dst
     *
     * @return string
     */
    public function getAbbreviatedName(bool $dst = false): string
    {}

    /**
     * @alias getAbbreviatedName
     *
     * Returns abbreviated name of the current timezone according to DST setting.
     *
     * @param bool $dst
     *
     * @return string
     */
    public function getAbbr(bool $dst = false): string
    {}

    /**
     * Get the offset as string "sHH:MM" (such as "+00:00" or "-12:30").
     */
    public function toOffsetName(?DateTimeInterface $date = null): string
    {}

    /**
     * Returns a new CarbonTimeZone object using the offset string instead of region string.
     */
    public function toOffsetTimeZone(?DateTimeInterface $date = null): static
    {}

    /**
     * Returns the first region string (such as "America/Toronto") that matches the current timezone or
     * false if no match is found.
     *
     * @see timezone_name_from_abbr native PHP function.
     */
    public function toRegionName(?DateTimeInterface $date = null, int $isDST = 1): ?string
    {}

    /**
     * Returns a new CarbonTimeZone object using the region string instead of offset string.
     */
    public function toRegionTimeZone(?DateTimeInterface $date = null): ?self
    {}

    /**
     * Cast to string (get timezone name).
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Return the type number:
     *
     * Type 1; A UTC offset, such as -0300
     * Type 2; A timezone abbreviation, such as GMT
     * Type 3: A timezone identifier, such as Europe/London
     */
    public function getType(): int
    {}

    /**
     * Create a CarbonTimeZone from mixed input.
     *
     * @param DateTimeZone|string|int|null $object
     *
     * @return false|static
     */
    public static function create($object = null)
    {}

    /**
     * Create a CarbonTimeZone from int/float hour offset.
     *
     * @param float $hourOffset number of hour of the timezone shift (can be decimal).
     *
     * @return false|static
     */
    public static function createFromHourOffset(float $hourOffset)
    {}

    /**
     * Create a CarbonTimeZone from int/float minute offset.
     *
     * @param float $minuteOffset number of total minutes of the timezone shift.
     *
     * @return false|static
     */
    public static function createFromMinuteOffset(float $minuteOffset)
    {}

    /**
     * Convert a total minutes offset into a standardized timezone offset string.
     *
     * @param float $minutes number of total minutes of the timezone shift.
     *
     * @return string
     */
    public static function getOffsetNameFromMinuteOffset(float $minutes): string
    {}

    private function resolveCarbon(?DateTimeInterface $date): DateTimeInterface
    {}
}
