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

use Carbon\CarbonConverterInterface;
use Carbon\CarbonInterface;
use Carbon\CarbonInterval;
use Carbon\Exceptions\InvalidFormatException;
use Carbon\Exceptions\InvalidIntervalException;
use Carbon\Exceptions\UnitException;
use Carbon\Exceptions\UnsupportedUnitException;
use Carbon\OverflowMode;
use Carbon\Unit;
use Closure;
use DateInterval;
use DateMalformedStringException;
use InvalidArgumentException;
use ReturnTypeWillChange;
use Time\Duration;

/**
 * Trait Units.
 *
 * Add, subtract and set units.
 */
trait Units
{
    /**
     * @deprecated Prefer to use add addUTCUnit() which more accurately defines what it's doing.
     *
     * Add seconds to the instance using timestamp. Positive $value travels
     * forward while negative $value travels into the past.
     *
     * @param string         $unit
     * @param int|float|null $value
     *
     * @return static
     */
    public function addRealUnit(string $unit, $value = 1): static
    {}

    /**
     * Add seconds to the instance using timestamp. Positive $value travels
     * forward while negative $value travels into the past.
     *
     * @param string         $unit
     * @param int|float|null $value
     *
     * @return static
     */
    public function addUTCUnit(string $unit, $value = 1): static
    {}

    /**
     * @deprecated Prefer to use add subUTCUnit() which more accurately defines what it's doing.
     *
     * Subtract seconds to the instance using timestamp. Positive $value travels
     * into the past while negative $value travels forward.
     *
     * @param string $unit
     * @param int    $value
     *
     * @return static
     */
    public function subRealUnit($unit, $value = 1): static
    {}

    /**
     * Subtract seconds to the instance using timestamp. Positive $value travels
     * into the past while negative $value travels forward.
     *
     * @param string $unit
     * @param int    $value
     *
     * @return static
     */
    public function subUTCUnit($unit, $value = 1): static
    {}

    /**
     * Returns true if a property can be changed via setter.
     *
     * @param string $unit
     *
     * @return bool
     */
    public static function isModifiableUnit($unit): bool
    {}

    /**
     * Call native PHP DateTime/DateTimeImmutable add() method.
     *
     * @param DateInterval $interval
     *
     * @return static
     */
    public function rawAdd(DateInterval $interval): static
    {}

    /**
     * Add given units or interval to the current instance.
     *
     * @example $date->add('hour', 3)
     * @example $date->add(15, 'days')
     * @example $date->add(CarbonInterval::days(4))
     *
     * @param Unit|int|string|DateInterval|Closure|CarbonConverterInterface|Duration $unit
     * @param Unit|int|float|string                                                  $value
     * @param OverflowMode|bool|null                                                 $overflow
     * @param int|null                                                               $anchorDay
     *
     * @return static
     */
    #[ReturnTypeWillChange]
    public function add($unit, $value = 1, OverflowMode|bool|null $overflow = null, ?int $anchorDay = null): static
    { func_get_args(); }

    /**
     * Add given units to the current instance.
     */
    public function addUnit(
        Unit|string $unit,
        $value = 1,
        OverflowMode|bool|null $overflow = null,
        ?int $anchorDay = null,
    ): static { func_get_args(); }

    /**
     * Subtract given units to the current instance.
     */
    public function subUnit(
        Unit|string $unit,
        $value = 1,
        OverflowMode|bool|null $overflow = null,
        ?int $anchorDay = null,
    ): static {}

    /**
     * Call native PHP DateTime/DateTimeImmutable sub() method.
     */
    public function rawSub(DateInterval $interval): static
    {}

    /**
     * Subtract given units or interval to the current instance.
     *
     * @example $date->sub('hour', 3)
     * @example $date->sub(15, 'days')
     * @example $date->sub(CarbonInterval::days(4))
     *
     * @param Unit|int|string|DateInterval|Duration|Closure|CarbonConverterInterface $unit
     * @param Unit|int|float|string                                                  $value
     * @param OverflowMode|bool|null                                                 $overflow
     * @param int|null                                                               $anchorDay
     *
     * @return static
     */
    #[ReturnTypeWillChange]
    public function sub($unit, $value = 1, OverflowMode|bool|null $overflow = null, ?int $anchorDay = null): static
    { func_get_args(); }

    /**
     * Subtract given units or interval to the current instance.
     *
     * @see sub()
     *
     * @param Unit|int|string|DateInterval|Duration $unit
     * @param Unit|int|float|string                 $value
     * @param OverflowMode|bool|null                $overflow
     * @param int|null                              $anchorDay
     *
     * @return static
     */
    public function subtract($unit, $value = 1, OverflowMode|bool|null $overflow = null, ?int $anchorDay = null): static
    { func_get_args(); }

    /**
     * Add given amount of time to the current date.
     *
     * @SuppressWarnings(ExcessiveParameterList)
     */
    private function doPlus(
        int $years = 0,
        int $months = 0,
        int|float $weeks = 0,
        int|float $days = 0,
        int|float $hours = 0,
        int|float $minutes = 0,
        int|float $seconds = 0,
        int|float $microseconds = 0,
        OverflowMode|bool|null $overflow = null,
        ?int $anchorDay = null,
    ): static {}

    /**
     * Subtract given amount of time to the current date.
     *
     * @SuppressWarnings(ExcessiveParameterList)
     */
    private function doMinus(
        int $years = 0,
        int $months = 0,
        int|float $weeks = 0,
        int|float $days = 0,
        int|float $hours = 0,
        int|float $minutes = 0,
        int|float $seconds = 0,
        int|float $microseconds = 0,
        OverflowMode|bool|null $overflow = null,
        ?int $anchorDay = null,
    ): static {}

    private function callPlusOrMinus(string $method, array $parameters): ?static
    {}

    private static function rawAddUnit(self $date, string $unit, int|float $value): ?static
    {}

    private static function getNumberAsString(int|float $value): string
    {}

    /**
     * Set current day of the instance to the passed value if it exits in the
     * current month, else set current day to the last day of the month.
     */
    public function setAnchorDay(int $anchorDay): static
    {}

    private static function disallowDecimalPart(mixed $value): void
    {}

    private function getOverflowMode(
        OverflowMode|bool|null $overflow = null,
        ?int $anchorDay = null,
    ): ?OverflowMode {}

    private function shouldUnitOverflow(string $unit): bool
    {}
}
