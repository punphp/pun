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
use Carbon\Exceptions\EndLessPeriodException;
use Carbon\Exceptions\InvalidCastException;
use Carbon\Exceptions\InvalidIntervalException;
use Carbon\Exceptions\InvalidPeriodDateException;
use Carbon\Exceptions\InvalidPeriodParameterException;
use Carbon\Exceptions\NotACarbonClassException;
use Carbon\Exceptions\NotAPeriodException;
use Carbon\Exceptions\PeriodFilterSafetyException;
use Carbon\Exceptions\UnknownGetterException;
use Carbon\Exceptions\UnknownMethodException;
use Carbon\Exceptions\UnreachableException;
use Carbon\Traits\DeprecatedPeriodProperties;
use Carbon\Traits\IntervalRounding;
use Carbon\Traits\LocalFactory;
use Carbon\Traits\Mixin;
use Carbon\Traits\Options;
use Carbon\Traits\ToStringFormat;
use Closure;
use Countable;
use DateInterval;
use DatePeriod;
use DateTime;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Generator;
use InvalidArgumentException;
use JsonSerializable;
use ReflectionException;
use ReturnTypeWillChange;
use RuntimeException;
use Throwable;
use Time\Duration;

// @codeCoverageIgnoreStart
require PHP_VERSION < 8.2
    ? __DIR__.'/../../lazy/Carbon/ProtectedDatePeriod.php'
    : __DIR__.'/../../lazy/Carbon/UnprotectedDatePeriod.php';
// @codeCoverageIgnoreEnd

/**
 * Substitution of DatePeriod with some modifications and many more features.
 *
 * @method static static|CarbonInterface start($date = null, $inclusive = null) Create instance specifying start date or modify the start date if called on an instance.
 * @method static static since($date = null, $inclusive = null) Alias for start().
 * @method static static sinceNow($inclusive = null) Create instance with start date set to now or set the start date to now if called on an instance.
 * @method static static|CarbonInterface end($date = null, $inclusive = null) Create instance specifying end date or modify the end date if called on an instance.
 * @method static static until($date = null, $inclusive = null) Alias for end().
 * @method static static untilNow($inclusive = null) Create instance with end date set to now or set the end date to now if called on an instance.
 * @method static static dates($start, $end = null) Create instance with start and end dates or modify the start and end dates if called on an instance.
 * @method static static between($start, $end = null) Create instance with start and end dates or modify the start and end dates if called on an instance.
 * @method static static recurrences($recurrences = null) Create instance with maximum number of recurrences or modify the number of recurrences if called on an instance.
 * @method static static times($recurrences = null) Alias for recurrences().
 * @method static static|int|null options($options = null) Create instance with options or modify the options if called on an instance.
 * @method static static toggle($options, $state = null) Create instance with options toggled on or off, or toggle options if called on an instance.
 * @method static static filter($callback, $name = null) Create instance with filter added to the stack or append a filter if called on an instance.
 * @method static static push($callback, $name = null) Alias for filter().
 * @method static static prepend($callback, $name = null) Create instance with filter prepended to the stack or prepend a filter if called on an instance.
 * @method static static|array filters(array $filters = []) Create instance with filters stack or replace the whole filters stack if called on an instance.
 * @method static static|CarbonInterval interval($interval = null) Create instance with given date interval or modify the interval if called on an instance.
 * @method static static each($interval) Create instance with given date interval or modify the interval if called on an instance.
 * @method static static every($interval) Create instance with given date interval or modify the interval if called on an instance.
 * @method static static step($interval) Create instance with given date interval or modify the interval if called on an instance.
 * @method static static stepBy($interval) Create instance with given date interval or modify the interval if called on an instance.
 * @method static static invert() Create instance with inverted date interval or invert the interval if called on an instance.
 * @method static static years($years = 1) Create instance specifying a number of years for date interval or replace the interval by the given a number of years if called on an instance.
 * @method static static year($years = 1) Alias for years().
 * @method static static months($months = 1) Create instance specifying a number of months for date interval or replace the interval by the given a number of months if called on an instance.
 * @method static static month($months = 1) Alias for months().
 * @method static static weeks($weeks = 1) Create instance specifying a number of weeks for date interval or replace the interval by the given a number of weeks if called on an instance.
 * @method static static week($weeks = 1) Alias for weeks().
 * @method static static days($days = 1) Create instance specifying a number of days for date interval or replace the interval by the given a number of days if called on an instance.
 * @method static static dayz($days = 1) Alias for days().
 * @method static static day($days = 1) Alias for days().
 * @method static static hours($hours = 1) Create instance specifying a number of hours for date interval or replace the interval by the given a number of hours if called on an instance.
 * @method static static hour($hours = 1) Alias for hours().
 * @method static static minutes($minutes = 1) Create instance specifying a number of minutes for date interval or replace the interval by the given a number of minutes if called on an instance.
 * @method static static minute($minutes = 1) Alias for minutes().
 * @method static static seconds($seconds = 1) Create instance specifying a number of seconds for date interval or replace the interval by the given a number of seconds if called on an instance.
 * @method static static second($seconds = 1) Alias for seconds().
 * @method static static milliseconds($milliseconds = 1) Create instance specifying a number of milliseconds for date interval or replace the interval by the given a number of milliseconds if called on an instance.
 * @method static static millisecond($milliseconds = 1) Alias for milliseconds().
 * @method static static microseconds($microseconds = 1) Create instance specifying a number of microseconds for date interval or replace the interval by the given a number of microseconds if called on an instance.
 * @method static static microsecond($microseconds = 1) Alias for microseconds().
 * @method $this roundYear(float $precision = 1, string $function = "round") Round the current instance year with given precision using the given function.
 * @method $this roundYears(float $precision = 1, string $function = "round") Round the current instance year with given precision using the given function.
 * @method $this floorYear(float $precision = 1) Truncate the current instance year with given precision.
 * @method $this floorYears(float $precision = 1) Truncate the current instance year with given precision.
 * @method $this ceilYear(float $precision = 1) Ceil the current instance year with given precision.
 * @method $this ceilYears(float $precision = 1) Ceil the current instance year with given precision.
 * @method $this roundMonth(float $precision = 1, string $function = "round") Round the current instance month with given precision using the given function.
 * @method $this roundMonths(float $precision = 1, string $function = "round") Round the current instance month with given precision using the given function.
 * @method $this floorMonth(float $precision = 1) Truncate the current instance month with given precision.
 * @method $this floorMonths(float $precision = 1) Truncate the current instance month with given precision.
 * @method $this ceilMonth(float $precision = 1) Ceil the current instance month with given precision.
 * @method $this ceilMonths(float $precision = 1) Ceil the current instance month with given precision.
 * @method $this roundWeek(float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this roundWeeks(float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this floorWeek(float $precision = 1) Truncate the current instance day with given precision.
 * @method $this floorWeeks(float $precision = 1) Truncate the current instance day with given precision.
 * @method $this ceilWeek(float $precision = 1) Ceil the current instance day with given precision.
 * @method $this ceilWeeks(float $precision = 1) Ceil the current instance day with given precision.
 * @method $this roundDay(float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this roundDays(float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this floorDay(float $precision = 1) Truncate the current instance day with given precision.
 * @method $this floorDays(float $precision = 1) Truncate the current instance day with given precision.
 * @method $this ceilDay(float $precision = 1) Ceil the current instance day with given precision.
 * @method $this ceilDays(float $precision = 1) Ceil the current instance day with given precision.
 * @method $this roundHour(float $precision = 1, string $function = "round") Round the current instance hour with given precision using the given function.
 * @method $this roundHours(float $precision = 1, string $function = "round") Round the current instance hour with given precision using the given function.
 * @method $this floorHour(float $precision = 1) Truncate the current instance hour with given precision.
 * @method $this floorHours(float $precision = 1) Truncate the current instance hour with given precision.
 * @method $this ceilHour(float $precision = 1) Ceil the current instance hour with given precision.
 * @method $this ceilHours(float $precision = 1) Ceil the current instance hour with given precision.
 * @method $this roundMinute(float $precision = 1, string $function = "round") Round the current instance minute with given precision using the given function.
 * @method $this roundMinutes(float $precision = 1, string $function = "round") Round the current instance minute with given precision using the given function.
 * @method $this floorMinute(float $precision = 1) Truncate the current instance minute with given precision.
 * @method $this floorMinutes(float $precision = 1) Truncate the current instance minute with given precision.
 * @method $this ceilMinute(float $precision = 1) Ceil the current instance minute with given precision.
 * @method $this ceilMinutes(float $precision = 1) Ceil the current instance minute with given precision.
 * @method $this roundSecond(float $precision = 1, string $function = "round") Round the current instance second with given precision using the given function.
 * @method $this roundSeconds(float $precision = 1, string $function = "round") Round the current instance second with given precision using the given function.
 * @method $this floorSecond(float $precision = 1) Truncate the current instance second with given precision.
 * @method $this floorSeconds(float $precision = 1) Truncate the current instance second with given precision.
 * @method $this ceilSecond(float $precision = 1) Ceil the current instance second with given precision.
 * @method $this ceilSeconds(float $precision = 1) Ceil the current instance second with given precision.
 * @method $this roundMillennium(float $precision = 1, string $function = "round") Round the current instance millennium with given precision using the given function.
 * @method $this roundMillennia(float $precision = 1, string $function = "round") Round the current instance millennium with given precision using the given function.
 * @method $this floorMillennium(float $precision = 1) Truncate the current instance millennium with given precision.
 * @method $this floorMillennia(float $precision = 1) Truncate the current instance millennium with given precision.
 * @method $this ceilMillennium(float $precision = 1) Ceil the current instance millennium with given precision.
 * @method $this ceilMillennia(float $precision = 1) Ceil the current instance millennium with given precision.
 * @method $this roundCentury(float $precision = 1, string $function = "round") Round the current instance century with given precision using the given function.
 * @method $this roundCenturies(float $precision = 1, string $function = "round") Round the current instance century with given precision using the given function.
 * @method $this floorCentury(float $precision = 1) Truncate the current instance century with given precision.
 * @method $this floorCenturies(float $precision = 1) Truncate the current instance century with given precision.
 * @method $this ceilCentury(float $precision = 1) Ceil the current instance century with given precision.
 * @method $this ceilCenturies(float $precision = 1) Ceil the current instance century with given precision.
 * @method $this roundDecade(float $precision = 1, string $function = "round") Round the current instance decade with given precision using the given function.
 * @method $this roundDecades(float $precision = 1, string $function = "round") Round the current instance decade with given precision using the given function.
 * @method $this floorDecade(float $precision = 1) Truncate the current instance decade with given precision.
 * @method $this floorDecades(float $precision = 1) Truncate the current instance decade with given precision.
 * @method $this ceilDecade(float $precision = 1) Ceil the current instance decade with given precision.
 * @method $this ceilDecades(float $precision = 1) Ceil the current instance decade with given precision.
 * @method $this roundQuarter(float $precision = 1, string $function = "round") Round the current instance quarter with given precision using the given function.
 * @method $this roundQuarters(float $precision = 1, string $function = "round") Round the current instance quarter with given precision using the given function.
 * @method $this floorQuarter(float $precision = 1) Truncate the current instance quarter with given precision.
 * @method $this floorQuarters(float $precision = 1) Truncate the current instance quarter with given precision.
 * @method $this ceilQuarter(float $precision = 1) Ceil the current instance quarter with given precision.
 * @method $this ceilQuarters(float $precision = 1) Ceil the current instance quarter with given precision.
 * @method $this roundMillisecond(float $precision = 1, string $function = "round") Round the current instance millisecond with given precision using the given function.
 * @method $this roundMilliseconds(float $precision = 1, string $function = "round") Round the current instance millisecond with given precision using the given function.
 * @method $this floorMillisecond(float $precision = 1) Truncate the current instance millisecond with given precision.
 * @method $this floorMilliseconds(float $precision = 1) Truncate the current instance millisecond with given precision.
 * @method $this ceilMillisecond(float $precision = 1) Ceil the current instance millisecond with given precision.
 * @method $this ceilMilliseconds(float $precision = 1) Ceil the current instance millisecond with given precision.
 * @method $this roundMicrosecond(float $precision = 1, string $function = "round") Round the current instance microsecond with given precision using the given function.
 * @method $this roundMicroseconds(float $precision = 1, string $function = "round") Round the current instance microsecond with given precision using the given function.
 * @method $this floorMicrosecond(float $precision = 1) Truncate the current instance microsecond with given precision.
 * @method $this floorMicroseconds(float $precision = 1) Truncate the current instance microsecond with given precision.
 * @method $this ceilMicrosecond(float $precision = 1) Ceil the current instance microsecond with given precision.
 * @method $this ceilMicroseconds(float $precision = 1) Ceil the current instance microsecond with given precision.
 *
 * @mixin DeprecatedPeriodProperties
 *
 * @SuppressWarnings(TooManyFields)
 * @SuppressWarnings(CamelCasePropertyName)
 * @SuppressWarnings(CouplingBetweenObjects)
 */
class CarbonPeriod extends DatePeriodBase implements Countable, JsonSerializable, UnitValue
{
    use LocalFactory;
    use IntervalRounding;
    use Mixin {
        Mixin::mixin as baseMixin;
    }
    use Options {
        Options::__debugInfo as baseDebugInfo;
    }
    use ToStringFormat;

    /**
     * Built-in filter for limit by recurrences.
     *
     * @var callable
     */
    public const RECURRENCES_FILTER = [self::class, 'filterRecurrences'];

    /**
     * Built-in filter for limit to an end.
     *
     * @var callable
     */
    public const END_DATE_FILTER = [self::class, 'filterEndDate'];

    /**
     * Special value which can be returned by filters to end iteration. Also a filter.
     *
     * @var callable
     */
    public const END_ITERATION = [self::class, 'endIteration'];

    /**
     * Exclude end date from iteration.
     *
     * @var int
     */
    public const EXCLUDE_END_DATE = 8;

    /**
     * Yield CarbonImmutable instances.
     *
     * @var int
     */
    public const IMMUTABLE = 4;

    /**
     * Number of maximum attempts before giving up on finding next valid date.
     *
     * @var int
     */
    public const NEXT_MAX_ATTEMPTS = 1000;

    /**
     * Number of maximum attempts before giving up on finding end date.
     *
     * @var int
     */
    public const END_MAX_ATTEMPTS = 10000;

    /**
     * Default date class of iteration items.
     *
     * @var string
     */
    protected const DEFAULT_DATE_CLASS = Carbon::class;

    /**
     * The registered macros.
     */
    protected static array $macros = [];

    /**
     * Date class of iteration items.
     */
    protected string $dateClass = Carbon::class;

    /**
     * Underlying date interval instance. Always present, one day by default.
     */
    protected ?CarbonInterval $dateInterval = null;

    /**
     * True once __construct is finished.
     */
    protected bool $constructed = false;

    /**
     * Whether current date interval was set by default.
     */
    protected bool $isDefaultInterval = false;

    /**
     * The filters stack.
     */
    protected array $filters = [];

    /**
     * Period start date. Applied on rewind. Always present, now by default.
     */
    protected ?CarbonInterface $startDate = null;

    /**
     * Period end date. For inverted interval should be before the start date. Applied via a filter.
     */
    protected ?CarbonInterface $endDate = null;

    /**
     * Limit for number of recurrences. Applied via a filter.
     */
    protected int|float|null $carbonRecurrences = null;

    /**
     * Iteration options.
     */
    protected ?int $options = null;

    /**
     * Index of current date. Always sequential, even if some dates are skipped by filters.
     * Equal to null only before the first iteration.
     */
    protected int $key = 0;

    /**
     * Current date. May temporarily hold unaccepted value when looking for a next valid date.
     * Equal to null only before the first iteration.
     */
    protected ?CarbonInterface $carbonCurrent = null;

    /**
     * Timezone of current date. Taken from the start date.
     */
    protected ?DateTimeZone $timezone = null;

    /**
     * The cached validation result for current date.
     */
    protected array|string|bool|null $validationResult = null;

    /**
     * Timezone handler for settings() method.
     */
    protected DateTimeZone|string|int|null $timezoneSetting = null;

    public function getIterator(): Generator
    { yield; }

    /**
     * Make a CarbonPeriod instance from given variable if possible.
     */
    public static function make(mixed $var): ?static
    {}

    /**
     * Create a new instance from a DatePeriod or CarbonPeriod object.
     */
    public static function instance(mixed $period): static
    {}

    /**
     * Create a new instance.
     */
    public static function create(...$params): static
    {}

    /**
     * Create a new instance from an array of parameters.
     */
    public static function createFromArray(array $params): static
    {}

    /**
     * Create CarbonPeriod from ISO 8601 string.
     */
    public static function createFromIso(string $iso, ?int $options = null): static
    {}

    public static function createFromISO8601String(string $iso, ?int $options = null): static
    {}

    public static function monthly(
        DateTimeInterface|string|int|float|null $start = null,
        DateTimeInterface|string|int|float|null $end = null,
        ?int $recurrences = null,
        ?int $anchorDay = null,
        OverflowMode $mode = OverflowMode::AnchorDay,
        ?int $options = null,
    ): static {}

    public static function quarterly(
        DateTimeInterface|string|int|float|null $start = null,
        DateTimeInterface|string|int|float|null $end = null,
        ?int $recurrences = null,
        ?int $anchorDay = null,
        OverflowMode $mode = OverflowMode::AnchorDay,
        ?int $options = null,
    ): static {}

    public static function yearly(
        DateTimeInterface|string|int|float|null $start = null,
        DateTimeInterface|string|int|float|null $end = null,
        ?int $recurrences = null,
        ?int $anchorDay = null,
        OverflowMode $mode = OverflowMode::AnchorDay,
        ?int $options = null,
    ): static {}

    /**
     * Return whether the given interval contains non-zero value of any time unit.
     */
    protected static function intervalHasTime(DateInterval $interval): bool
    {}

    /**
     * Return whether given variable is an ISO 8601 specification.
     *
     * Note: Check is very basic, as actual validation will be done later when parsing.
     * We just want to ensure that variable is not any other type of valid parameter.
     */
    protected static function isIso8601(mixed $var): bool
    {}

    /**
     * Parse given ISO 8601 string into an array of arguments.
     *
     * @SuppressWarnings(ElseExpression)
     */
    protected static function parseIso8601(string $iso): array
    {}

    /**
     * Add missing parts of the target date from the source date.
     */
    protected static function addMissingParts(string $source, string $target): string
    {}

    private static function getStartAndEndForCyclePeriod(
        DateTimeInterface|string|int|float|null $start = null,
        DateTimeInterface|string|int|float|null $end = null,
        ?int $recurrences = null,
        ?int $anchorDay = null,
        OverflowMode $mode = OverflowMode::AnchorDay,
    ): array {}

    private static function getMonthInterval(
        DateTimeInterface|string|int|null $start = null,
        ?int $anchorDay = null,
        OverflowMode $mode = OverflowMode::AnchorDay,
    ): CarbonInterval {}

    private static function makeInterval(mixed $input): ?CarbonInterval
    {}

    private static function makeTimezone(mixed $input): ?CarbonTimeZone
    {}

    /**
     * Register a custom macro.
     *
     * Pass null macro to remove it.
     *
     * @example
     * ```
     * CarbonPeriod::macro('middle', function () {
     *   return $this->getStartDate()->average($this->getEndDate());
     * });
     * echo CarbonPeriod::since('2011-05-12')->until('2011-06-03')->middle();
     * ```
     *
     * @param-closure-this  static  $macro
     */
    public static function macro(string $name, ?callable $macro): void
    {}

    /**
     * Register macros from a mixin object.
     *
     * @example
     * ```
     * CarbonPeriod::mixin(new class {
     *   public function addDays() {
     *     return function ($count = 1) {
     *       return $this->setStartDate(
     *         $this->getStartDate()->addDays($count)
     *       )->setEndDate(
     *         $this->getEndDate()->addDays($count)
     *       );
     *     };
     *   }
     *   public function subDays() {
     *     return function ($count = 1) {
     *       return $this->setStartDate(
     *         $this->getStartDate()->subDays($count)
     *       )->setEndDate(
     *         $this->getEndDate()->subDays($count)
     *       );
     *     };
     *   }
     * });
     * echo CarbonPeriod::create('2000-01-01', '2000-02-01')->addDays(5)->subDays(3);
     * ```
     *
     * @throws ReflectionException
     */
    public static function mixin(object|string $mixin): void
    {}

    /**
     * Check if macro is registered.
     */
    public static function hasMacro(string $name): bool
    {}

    /**
     * Provide static proxy for instance aliases.
     */
    public static function __callStatic(string $method, array $parameters): mixed
    {}

    /**
     * CarbonPeriod constructor.
     *
     * @SuppressWarnings(ElseExpression)
     *
     * @throws InvalidArgumentException
     */
    public function __construct(...$arguments)
    {}

    /**
     * Get a copy of the instance.
     */
    public function copy(): static
    {}

    /**
     * Prepare the instance to be set (self if mutable to be mutated,
     * copy if immutable to generate a new instance).
     */
    protected function copyIfImmutable(): static
    {}

    /**
     * Get the getter for a property allowing both `DatePeriod` snakeCase and camelCase names.
     */
    protected function getGetter(string $name): ?callable
    {}

    /**
     * Get a property allowing both `DatePeriod` snakeCase and camelCase names.
     *
     * @param string $name
     *
     * @return bool|CarbonInterface|CarbonInterval|int|null
     */
    public function get(string $name)
    {}

    /**
     * Get a property allowing both `DatePeriod` snakeCase and camelCase names.
     *
     * @param string $name
     *
     * @return bool|CarbonInterface|CarbonInterval|int|null
     */
    public function __get(string $name)
    {}

    /**
     * Check if an attribute exists on the object
     *
     * @param string $name
     *
     * @return bool
     */
    public function __isset(string $name): bool
    {}

    /**
     * @alias copy
     *
     * Get a copy of the instance.
     *
     * @return static
     */
    public function clone()
    {}

    /**
     * Set the iteration item class.
     *
     * @param string $dateClass
     *
     * @return static
     */
    public function setDateClass(string $dateClass)
    {}

    /**
     * Returns iteration item date class.
     *
     * @return string
     */
    public function getDateClass(): string
    {}

    /**
     * Change the period date interval.
     *
     * @param DateInterval|Duration|Unit|string|int $interval
     * @param Unit|string                           $unit     the unit of $interval if it's a number
     *
     * @throws InvalidIntervalException
     *
     * @return static
     */
    public function setDateInterval(mixed $interval, Unit|string|null $unit = null): static
    {}

    /**
     * Reset the date interval to the default value.
     *
     * Difference with simply setting interval to 1-day is that P1D will not appear when calling toIso8601String()
     * and also next adding to the interval won't include the default 1-day.
     */
    public function resetDateInterval(): static
    {}

    /**
     * Invert the period date interval.
     */
    public function invertDateInterval(): static
    {}

    /**
     * Set start and end date.
     *
     * @param DateTime|DateTimeInterface|string      $start
     * @param DateTime|DateTimeInterface|string|null $end
     *
     * @return static
     */
    public function setDates(mixed $start, mixed $end): static
    {}

    /**
     * Change the period options.
     *
     * @param int|null $options
     *
     * @return static
     */
    public function setOptions(?int $options): static
    {}

    /**
     * Get the period options.
     */
    public function getOptions(): int
    {}

    /**
     * Toggle given options on or off.
     *
     * @param int       $options
     * @param bool|null $state
     *
     * @throws InvalidArgumentException
     *
     * @return static
     */
    public function toggleOptions(int $options, ?bool $state = null): static
    {}

    /**
     * Toggle EXCLUDE_START_DATE option.
     */
    public function excludeStartDate(bool $state = true): static
    {}

    /**
     * Toggle EXCLUDE_END_DATE option.
     */
    public function excludeEndDate(bool $state = true): static
    {}

    /**
     * Get the underlying date interval.
     */
    public function getDateInterval(): CarbonInterval
    {}

    /**
     * Get start date of the period.
     *
     * @param string|null $rounding Optional rounding 'floor', 'ceil', 'round' using the period interval.
     */
    public function getStartDate(?string $rounding = null): CarbonInterface
    {}

    /**
     * Get end date of the period.
     *
     * @param string|null $rounding Optional rounding 'floor', 'ceil', 'round' using the period interval.
     */
    public function getEndDate(?string $rounding = null): ?CarbonInterface
    {}

    /**
     * Get number of recurrences.
     */
    #[ReturnTypeWillChange]
    public function getRecurrences(): int|float|null
    {}

    /**
     * Returns true if the start date should be excluded.
     */
    public function isStartExcluded(): bool
    {}

    /**
     * Returns true if the end date should be excluded.
     */
    public function isEndExcluded(): bool
    {}

    /**
     * Returns true if the start date should be included.
     */
    public function isStartIncluded(): bool
    {}

    /**
     * Returns true if the end date should be included.
     */
    public function isEndIncluded(): bool
    {}

    /**
     * Return the start if it's included by option, else return the start + 1 period interval.
     */
    public function getIncludedStartDate(): CarbonInterface
    {}

    /**
     * Return the end if it's included by option, else return the end - 1 period interval.
     * Warning: if the period has no fixed end, this method will iterate the period to calculate it.
     */
    public function getIncludedEndDate(): CarbonInterface
    {}

    /**
     * Add a filter to the stack.
     *
     * @SuppressWarnings(UnusedFormalParameter)
     */
    public function addFilter(callable|string $callback, ?string $name = null): static
    { func_get_args(); }

    /**
     * Prepend a filter to the stack.
     *
     * @SuppressWarnings(UnusedFormalParameter)
     */
    public function prependFilter(callable|string $callback, ?string $name = null): static
    { func_get_args(); }

    /**
     * Remove a filter by instance or name.
     */
    public function removeFilter(callable|string $filter): static
    {}

    /**
     * Return whether given instance or name is in the filter stack.
     */
    public function hasFilter(callable|string $filter): bool
    {}

    /**
     * Get filters stack.
     */
    public function getFilters(): array
    {}

    /**
     * Set filters stack.
     */
    public function setFilters(array $filters): static
    {}

    /**
     * Reset filters stack.
     */
    public function resetFilters(): static
    {}

    /**
     * Add a recurrences filter (set maximum number of recurrences).
     *
     * @throws InvalidArgumentException
     */
    public function setRecurrences(int|float|null $recurrences): static
    {}

    /**
     * Change the period start date.
     *
     * @param DateTime|DateTimeInterface|string $date
     * @param bool|null                         $inclusive
     *
     * @throws InvalidPeriodDateException
     *
     * @return static
     */
    public function setStartDate(mixed $date, ?bool $inclusive = null): static
    {}

    /**
     * Change the period end date.
     *
     * @param DateTime|DateTimeInterface|string|null $date
     * @param bool|null                              $inclusive
     *
     * @throws \InvalidArgumentException
     *
     * @return static
     */
    public function setEndDate(mixed $date, ?bool $inclusive = null): static
    {}

    /**
     * Check if the current position is valid.
     */
    public function valid(): bool
    {}

    /**
     * Return the current key.
     */
    public function key(): ?int
    {}

    /**
     * Return the current date.
     */
    public function current(): ?CarbonInterface
    {}

    /**
     * Move forward to the next date.
     *
     * @throws RuntimeException
     */
    public function next(): void
    {}

    /**
     * Rewind to the start date.
     *
     * Iterating over a date in the UTC timezone avoids bug during backward DST change.
     *
     * @see https://bugs.php.net/bug.php?id=72255
     * @see https://bugs.php.net/bug.php?id=74274
     * @see https://wiki.php.net/rfc/datetime_and_daylight_saving_time
     *
     * @throws RuntimeException
     */
    public function rewind(): void
    {}

    /**
     * Skip iterations and returns iteration state (false if ended, true if still valid).
     *
     * @param int $count steps number to skip (1 by default)
     *
     * @return bool
     */
    public function skip(int $count = 1): bool
    {}

    /**
     * Format the date period as ISO 8601.
     */
    public function toIso8601String(): string
    {}

    /**
     * Convert the date period into a string.
     */
    public function toString(): string
    {}

    /**
     * Format the date period as ISO 8601.
     */
    public function spec(): string
    {}

    /**
     * Cast the current instance into the given class.
     *
     * @param string $className The $className::instance() method will be called to cast the current object.
     *
     * @return DatePeriod|object
     */
    public function cast(string $className): object
    {}

    /**
     * Return native DatePeriod PHP object matching the current instance.
     *
     * @example
     * ```
     * var_dump(CarbonPeriod::create('2021-01-05', '2021-02-15')->toDatePeriod());
     * ```
     */
    public function toDatePeriod(): DatePeriod
    {}

    /**
     * Return `true` if the period has no custom filter and is guaranteed to be endless.
     *
     * Note that we can't check if a period is endless as soon as it has custom filters
     * because filters can emit `CarbonPeriod::END_ITERATION` to stop the iteration in
     * a way we can't predict without actually iterating the period.
     */
    public function isUnfilteredAndEndLess(): bool
    {}

    /**
     * Convert the date period into an array without changing current iteration state.
     *
     * @return CarbonInterface[]
     */
    public function toArray(): array
    {}

    /**
     * Count dates in the date period.
     */
    public function count(): int
    {}

    /**
     * Return the first date in the date period.
     */
    public function first(): ?CarbonInterface
    {}

    /**
     * Return the last date in the date period.
     */
    public function last(): ?CarbonInterface
    {}

    /**
     * Convert the date period into a string.
     */
    public function __toString(): string
    {}

    /**
     * Add aliases for setters.
     *
     * CarbonPeriod::days(3)->hours(5)->invert()
     *     ->sinceNow()->until('2010-01-10')
     *     ->filter(...)
     *     ->count()
     *
     * Note: We use magic method to let static and instance aliases with the same names.
     */
    public function __call(string $method, array $parameters): mixed
    {}

    /**
     * Set the instance's timezone from a string or object and apply it to start/end.
     */
    public function setTimezone(DateTimeZone|string|int $timezone): static
    {}

    /**
     * Set the instance's timezone from a string or object and add/subtract the offset difference to start/end.
     */
    public function shiftTimezone(DateTimeZone|string|int $timezone): static
    {}

    /**
     * Returns the end is set, else calculated from start and recurrences.
     *
     * @param string|null $rounding Optional rounding 'floor', 'ceil', 'round' using the period interval.
     *
     * @return CarbonInterface
     */
    public function calculateEnd(?string $rounding = null): CarbonInterface
    {}

    private function getEndFromRecurrences(): ?CarbonInterface
    {}

    private function iterateUntilEnd(): ?CarbonInterface
    {}

    /**
     * Returns true if the current period overlaps the given one (if 1 parameter passed)
     * or the period between 2 dates (if 2 parameters passed).
     *
     * @param CarbonPeriod|\DateTimeInterface|Carbon|CarbonImmutable|string $rangeOrRangeStart
     * @param \DateTimeInterface|Carbon|CarbonImmutable|string|null         $rangeEnd
     *
     * @return bool
     */
    public function overlaps(mixed $rangeOrRangeStart, mixed $rangeEnd = null): bool
    {}

    /**
     * Execute a given function on each date of the period.
     *
     * @example
     * ```
     * Carbon::create('2020-11-29')->daysUntil('2020-12-24')->forEach(function (Carbon $date) {
     *   echo $date->diffInDays('2020-12-25')." days before Christmas!\n";
     * });
     * ```
     */
    public function forEach(callable $callback): void
    {}

    /**
     * Execute a given function on each date of the period and yield the result of this function.
     *
     * @example
     * ```
     * $period = Carbon::create('2020-11-29')->daysUntil('2020-12-24');
     * echo implode("\n", iterator_to_array($period->map(function (Carbon $date) {
     *   return $date->diffInDays('2020-12-25').' days before Christmas!';
     * })));
     * ```
     */
    public function map(callable $callback): Generator
    { yield; }

    /**
     * Determines if the instance is equal to another.
     * Warning: if options differ, instances will never be equal.
     *
     * @see equalTo()
     */
    public function eq(mixed $period): bool
    {}

    /**
     * Determines if the instance is equal to another.
     * Warning: if options differ, instances will never be equal.
     */
    public function equalTo(mixed $period): bool
    {}

    /**
     * Determines if the instance is not equal to another.
     * Warning: if options differ, instances will never be equal.
     *
     * @see notEqualTo()
     */
    public function ne(mixed $period): bool
    {}

    /**
     * Determines if the instance is not equal to another.
     * Warning: if options differ, instances will never be equal.
     */
    public function notEqualTo(mixed $period): bool
    {}

    /**
     * Determines if the start date is before another given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function startsBefore(mixed $date = null): bool
    {}

    /**
     * Determines if the start date is before or the same as a given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function startsBeforeOrAt(mixed $date = null): bool
    {}

    /**
     * Determines if the start date is after another given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function startsAfter(mixed $date = null): bool
    {}

    /**
     * Determines if the start date is after or the same as a given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function startsAfterOrAt(mixed $date = null): bool
    {}

    /**
     * Determines if the start date is the same as a given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function startsAt(mixed $date = null): bool
    {}

    /**
     * Determines if the end date is before another given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function endsBefore(mixed $date = null): bool
    {}

    /**
     * Determines if the end date is before or the same as a given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function endsBeforeOrAt(mixed $date = null): bool
    {}

    /**
     * Determines if the end date is after another given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function endsAfter(mixed $date = null): bool
    {}

    /**
     * Determines if the end date is after or the same as a given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function endsAfterOrAt(mixed $date = null): bool
    {}

    /**
     * Determines if the end date is the same as a given date.
     * (Rather start/end are included by options is ignored.)
     */
    public function endsAt(mixed $date = null): bool
    {}

    /**
     * Return true if start date is now or later.
     * (Rather start/end are included by options is ignored.)
     */
    public function isStarted(): bool
    {}

    /**
     * Return true if end date is now or later.
     * (Rather start/end are included by options is ignored.)
     */
    public function isEnded(): bool
    {}

    /**
     * Return true if now is between start date (included) and end date (excluded).
     * (Rather start/end are included by options is ignored.)
     */
    public function isInProgress(): bool
    {}

    /**
     * Round the current instance at the given unit with given precision if specified and the given function.
     */
    public function roundUnit(
        string $unit,
        DateInterval|float|int|string|null $precision = 1,
        callable|string $function = 'round',
    ): static {}

    /**
     * Truncate the current instance at the given unit with given precision if specified.
     */
    public function floorUnit(string $unit, DateInterval|float|int|string|null $precision = 1): static
    {}

    /**
     * Ceil the current instance at the given unit with given precision if specified.
     */
    public function ceilUnit(string $unit, DateInterval|float|int|string|null $precision = 1): static
    {}

    /**
     * Round the current instance second with given precision if specified (else period interval is used).
     */
    public function round(
        DateInterval|float|int|string|null $precision = null,
        callable|string $function = 'round',
    ): static {}

    /**
     * Round the current instance second with given precision if specified (else period interval is used).
     */
    public function floor(DateInterval|float|int|string|null $precision = null): static
    {}

    /**
     * Ceil the current instance second with given precision if specified (else period interval is used).
     */
    public function ceil(DateInterval|float|int|string|null $precision = null): static
    {}

    /**
     * Specify data which should be serialized to JSON.
     *
     * @link https://php.net/manual/en/jsonserializable.jsonserialize.php
     *
     * @return CarbonInterface[]
     */
    public function jsonSerialize(): array
    {}

    /**
     * Return true if the given date is between start and end.
     */
    public function contains(mixed $date = null): bool
    {}

    /**
     * Return true if the current period follows a given other period (with no overlap).
     * For instance, [2019-08-01 -> 2019-08-12] follows [2019-07-29 -> 2019-07-31]
     * Note than in this example, follows() would be false if 2019-08-01 or 2019-07-31 was excluded by options.
     */
    public function follows(mixed $period, mixed ...$arguments): bool
    {}

    /**
     * Return true if the given other period follows the current one (with no overlap).
     * For instance, [2019-07-29 -> 2019-07-31] is followed by [2019-08-01 -> 2019-08-12]
     * Note than in this example, isFollowedBy() would be false if 2019-08-01 or 2019-07-31 was excluded by options.
     */
    public function isFollowedBy(mixed $period, mixed ...$arguments): bool
    {}

    /**
     * Return true if the given period either follows or is followed by the current one.
     *
     * @see follows()
     * @see isFollowedBy()
     */
    public function isConsecutiveWith(mixed $period, mixed ...$arguments): bool
    {}

    public function __debugInfo(): array
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * Update properties after removing built-in filters.
     */
    protected function updateInternalState(): void
    {}

    /**
     * Create a filter tuple from raw parameters.
     *
     * Will create an automatic filter callback for one of Carbon's is* methods.
     */
    protected function createFilterTuple(array $parameters): array
    {}

    /**
     * Return whether given callable is a string pointing to one of Carbon's is* methods
     * and should be automatically converted to a filter callback.
     */
    protected function isCarbonPredicateMethod(callable|string $callable): bool
    {}

    /**
     * Recurrences filter callback (limits number of recurrences).
     *
     * @SuppressWarnings(UnusedFormalParameter)
     */
    protected function filterRecurrences(CarbonInterface $current, int $key): bool|callable
    {}

    /**
     * End date filter callback.
     *
     * @return bool|static::END_ITERATION
     */
    protected function filterEndDate(CarbonInterface $current): bool|callable
    {}

    /**
     * End iteration filter callback.
     *
     * @return static::END_ITERATION
     */
    protected function endIteration(): callable
    {}

    /**
     * Handle change of the parameters.
     */
    protected function handleChangedParameters(): void
    {}

    /**
     * Synchronize the native DatePeriod properties with the current state.
     */
    protected function syncNativePeriod(): void
    {}

    /**
     * Validate current date and stop iteration when necessary.
     *
     * Returns true when current date is valid, false if it is not, or static::END_ITERATION
     * when iteration should be stopped.
     *
     * @return bool|static::END_ITERATION
     */
    protected function validateCurrentDate(): bool|callable
    {}

    /**
     * Check whether current value and key pass all the filters.
     *
     * @return bool|static::END_ITERATION
     */
    protected function checkFilters(): bool|callable
    {}

    /**
     * Prepare given date to be returned to the external logic.
     *
     * @param CarbonInterface $date
     *
     * @return CarbonInterface
     */
    protected function prepareForReturn(CarbonInterface $date)
    {}

    /**
     * Keep incrementing the current date until a valid date is found or the iteration is ended.
     *
     * @throws RuntimeException
     */
    protected function incrementCurrentDateUntilValid(): void
    {}

    /**
     * Call given macro.
     */
    protected function callMacro(string $name, array $parameters): mixed
    {}

    /**
     * Return the Carbon instance passed through, a now instance in the same timezone
     * if null given or parse the input if string given.
     *
     * @param \Carbon\Carbon|\Carbon\CarbonPeriod|\Carbon\CarbonInterval|\Time\Duration|\DateInterval|\DatePeriod|\DateTimeInterface|string|null $date
     *
     * @return \Carbon\CarbonInterface
     */
    protected function resolveCarbon($date = null)
    {}

    /**
     * Resolve passed arguments or DatePeriod to a CarbonPeriod object.
     */
    protected function resolveCarbonPeriod(mixed $period, mixed ...$arguments): self
    {}

    private function carbonOrResolve(mixed $dateTime): CarbonInterface
    {}

    private function orderCouple($first, $second): array
    {}

    private function makeDateTime($value): ?DateTimeInterface
    {}

    private function isInfiniteDate($date): bool
    {}

    private function rawDate($date): ?DateTimeInterface
    {}

    private static function setDefaultParameters(array &$parameters, array $defaults): void
    {}

    private function setFromAssociativeArray(array $parameters): void
    {}

    private function configureTimezone(DateTimeZone $timezone, array $sortedArguments, array $originalArguments): array
    {}

    private function initializeSerialization(array $values): void
    {}

    private static function addFilterOrHandleChangedParameters(
        self $period,
        array|callable|string $filter,
    ): self {}

    private static function makeCarbonInterval(mixed $interval, Unit|string|null $unit = null): ?CarbonInterval
    {}
}
