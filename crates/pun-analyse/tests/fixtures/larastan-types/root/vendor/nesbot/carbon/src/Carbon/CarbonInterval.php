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
use Carbon\Exceptions\BadFluentConstructorException;
use Carbon\Exceptions\BadFluentSetterException;
use Carbon\Exceptions\InvalidCastException;
use Carbon\Exceptions\InvalidFormatException;
use Carbon\Exceptions\InvalidIntervalException;
use Carbon\Exceptions\OutOfRangeException;
use Carbon\Exceptions\ParseErrorException;
use Carbon\Exceptions\UnitNotConfiguredException;
use Carbon\Exceptions\UnknownGetterException;
use Carbon\Exceptions\UnknownSetterException;
use Carbon\Exceptions\UnknownUnitException;
use Carbon\Traits\IntervalRounding;
use Carbon\Traits\IntervalStep;
use Carbon\Traits\LocalFactory;
use Carbon\Traits\MagicParameter;
use Carbon\Traits\Mixin;
use Carbon\Traits\Options;
use Carbon\Traits\ToStringFormat;
use Closure;
use DateInterval;
use DateTime;
use DateTimeInterface;
use DateTimeZone;
use Exception;
use InvalidArgumentException;
use ReflectionException;
use ReturnTypeWillChange;
use RuntimeException;
use Symfony\Contracts\Translation\TranslatorInterface;
use Throwable;
use Time\Duration;

/**
 * A simple API extension for DateInterval.
 * The implementation provides helpers to handle weeks but only days are saved.
 * Weeks are calculated based on the total days of the current instance.
 *
 * @property int $years Year component of the current interval. (For P2Y6M, the value will be 2)
 * @property int $months Month component of the current interval. (For P1Y6M10D, the value will be 6)
 * @property int $weeks Week component of the current interval calculated from the days. (For P1Y6M17D, the value will be 2)
 * @property int $dayz Day component of the current interval (weeks * 7 + days). (For P6M17DT20H, the value will be 17)
 * @property int $hours Hour component of the current interval. (For P7DT20H5M, the value will be 20)
 * @property int $minutes Minute component of the current interval. (For PT20H5M30S, the value will be 5)
 * @property int $seconds Second component of the current interval. (CarbonInterval::minutes(2)->seconds(34)->microseconds(567_890)->seconds = 34)
 * @property int $milliseconds Milliseconds component of the current interval. (CarbonInterval::seconds(34)->microseconds(567_890)->milliseconds = 567)
 * @property int $microseconds Microseconds component of the current interval. (CarbonInterval::seconds(34)->microseconds(567_890)->microseconds = 567_890)
 * @property int $microExcludeMilli Remaining microseconds without the milliseconds.
 * @property int $dayzExcludeWeeks Total days remaining in the final week of the current instance (days % 7).
 * @property int $daysExcludeWeeks alias of dayzExcludeWeeks
 * @property-read float $totalYears Number of years equivalent to the interval. (For P1Y6M, the value will be 1.5)
 * @property-read float $totalMonths Number of months equivalent to the interval. (For P1Y6M10D, the value will be ~12.357)
 * @property-read float $totalWeeks Number of weeks equivalent to the interval. (For P6M17DT20H, the value will be ~26.548)
 * @property-read float $totalDays Number of days equivalent to the interval. (For P17DT20H, the value will be ~17.833)
 * @property-read float $totalDayz Alias for totalDays.
 * @property-read float $totalHours Number of hours equivalent to the interval. (For P1DT20H5M, the value will be ~44.083)
 * @property-read float $totalMinutes Number of minutes equivalent to the interval. (For PT20H5M30S, the value will be 1205.5)
 * @property-read float $totalSeconds Number of seconds equivalent to the interval. (CarbonInterval::minutes(2)->seconds(34)->microseconds(567_890)->totalSeconds = 154.567_890)
 * @property-read float $totalMilliseconds Number of milliseconds equivalent to the interval. (CarbonInterval::seconds(34)->microseconds(567_890)->totalMilliseconds = 34567.890)
 * @property-read float $totalMicroseconds Number of microseconds equivalent to the interval. (CarbonInterval::seconds(34)->microseconds(567_890)->totalMicroseconds = 34567890)
 * @property-read string $locale locale of the current instance
 *
 * @method static CarbonInterval years($years = 1) Create instance specifying a number of years or modify the number of years if called on an instance.
 * @method static CarbonInterval year($years = 1) Alias for years()
 * @method static CarbonInterval months($months = 1) Create instance specifying a number of months or modify the number of months if called on an instance.
 * @method static CarbonInterval month($months = 1) Alias for months()
 * @method static CarbonInterval weeks($weeks = 1) Create instance specifying a number of weeks or modify the number of weeks if called on an instance.
 * @method static CarbonInterval week($weeks = 1) Alias for weeks()
 * @method static CarbonInterval days($days = 1) Create instance specifying a number of days or modify the number of days if called on an instance.
 * @method static CarbonInterval dayz($days = 1) Alias for days()
 * @method static CarbonInterval daysExcludeWeeks($days = 1) Create instance specifying a number of days or modify the number of days (keeping the current number of weeks) if called on an instance.
 * @method static CarbonInterval dayzExcludeWeeks($days = 1) Alias for daysExcludeWeeks()
 * @method static CarbonInterval day($days = 1) Alias for days()
 * @method static CarbonInterval hours($hours = 1) Create instance specifying a number of hours or modify the number of hours if called on an instance.
 * @method static CarbonInterval hour($hours = 1) Alias for hours()
 * @method static CarbonInterval minutes($minutes = 1) Create instance specifying a number of minutes or modify the number of minutes if called on an instance.
 * @method static CarbonInterval minute($minutes = 1) Alias for minutes()
 * @method static CarbonInterval seconds($seconds = 1) Create instance specifying a number of seconds or modify the number of seconds if called on an instance.
 * @method static CarbonInterval second($seconds = 1) Alias for seconds()
 * @method static CarbonInterval milliseconds($milliseconds = 1) Create instance specifying a number of milliseconds or modify the number of milliseconds if called on an instance.
 * @method static CarbonInterval millisecond($milliseconds = 1) Alias for milliseconds()
 * @method static CarbonInterval microseconds($microseconds = 1) Create instance specifying a number of microseconds or modify the number of microseconds if called on an instance.
 * @method static CarbonInterval microsecond($microseconds = 1) Alias for microseconds()
 * @method $this addYears(int $years) Add given number of years to the current interval
 * @method $this subYears(int $years) Subtract given number of years to the current interval
 * @method $this addMonths(int $months) Add given number of months to the current interval
 * @method $this subMonths(int $months) Subtract given number of months to the current interval
 * @method $this addWeeks(int|float $weeks) Add given number of weeks to the current interval
 * @method $this subWeeks(int|float $weeks) Subtract given number of weeks to the current interval
 * @method $this addDays(int|float $days) Add given number of days to the current interval
 * @method $this subDays(int|float $days) Subtract given number of days to the current interval
 * @method $this addHours(int|float $hours) Add given number of hours to the current interval
 * @method $this subHours(int|float $hours) Subtract given number of hours to the current interval
 * @method $this addMinutes(int|float $minutes) Add given number of minutes to the current interval
 * @method $this subMinutes(int|float $minutes) Subtract given number of minutes to the current interval
 * @method $this addSeconds(int|float $seconds) Add given number of seconds to the current interval
 * @method $this subSeconds(int|float $seconds) Subtract given number of seconds to the current interval
 * @method $this addMilliseconds(int|float $milliseconds) Add given number of milliseconds to the current interval
 * @method $this subMilliseconds(int|float $milliseconds) Subtract given number of milliseconds to the current interval
 * @method $this addMicroseconds(int|float $microseconds) Add given number of microseconds to the current interval
 * @method $this subMicroseconds(int|float $microseconds) Subtract given number of microseconds to the current interval
 * @method $this roundYear(int|float $precision = 1, string $function = "round") Round the current instance year with given precision using the given function.
 * @method $this roundYears(int|float $precision = 1, string $function = "round") Round the current instance year with given precision using the given function.
 * @method $this floorYear(int|float $precision = 1) Truncate the current instance year with given precision.
 * @method $this floorYears(int|float $precision = 1) Truncate the current instance year with given precision.
 * @method $this ceilYear(int|float $precision = 1) Ceil the current instance year with given precision.
 * @method $this ceilYears(int|float $precision = 1) Ceil the current instance year with given precision.
 * @method $this roundMonth(int|float $precision = 1, string $function = "round") Round the current instance month with given precision using the given function.
 * @method $this roundMonths(int|float $precision = 1, string $function = "round") Round the current instance month with given precision using the given function.
 * @method $this floorMonth(int|float $precision = 1) Truncate the current instance month with given precision.
 * @method $this floorMonths(int|float $precision = 1) Truncate the current instance month with given precision.
 * @method $this ceilMonth(int|float $precision = 1) Ceil the current instance month with given precision.
 * @method $this ceilMonths(int|float $precision = 1) Ceil the current instance month with given precision.
 * @method $this roundWeek(int|float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this roundWeeks(int|float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this floorWeek(int|float $precision = 1) Truncate the current instance day with given precision.
 * @method $this floorWeeks(int|float $precision = 1) Truncate the current instance day with given precision.
 * @method $this ceilWeek(int|float $precision = 1) Ceil the current instance day with given precision.
 * @method $this ceilWeeks(int|float $precision = 1) Ceil the current instance day with given precision.
 * @method $this roundDay(int|float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this roundDays(int|float $precision = 1, string $function = "round") Round the current instance day with given precision using the given function.
 * @method $this floorDay(int|float $precision = 1) Truncate the current instance day with given precision.
 * @method $this floorDays(int|float $precision = 1) Truncate the current instance day with given precision.
 * @method $this ceilDay(int|float $precision = 1) Ceil the current instance day with given precision.
 * @method $this ceilDays(int|float $precision = 1) Ceil the current instance day with given precision.
 * @method $this roundHour(int|float $precision = 1, string $function = "round") Round the current instance hour with given precision using the given function.
 * @method $this roundHours(int|float $precision = 1, string $function = "round") Round the current instance hour with given precision using the given function.
 * @method $this floorHour(int|float $precision = 1) Truncate the current instance hour with given precision.
 * @method $this floorHours(int|float $precision = 1) Truncate the current instance hour with given precision.
 * @method $this ceilHour(int|float $precision = 1) Ceil the current instance hour with given precision.
 * @method $this ceilHours(int|float $precision = 1) Ceil the current instance hour with given precision.
 * @method $this roundMinute(int|float $precision = 1, string $function = "round") Round the current instance minute with given precision using the given function.
 * @method $this roundMinutes(int|float $precision = 1, string $function = "round") Round the current instance minute with given precision using the given function.
 * @method $this floorMinute(int|float $precision = 1) Truncate the current instance minute with given precision.
 * @method $this floorMinutes(int|float $precision = 1) Truncate the current instance minute with given precision.
 * @method $this ceilMinute(int|float $precision = 1) Ceil the current instance minute with given precision.
 * @method $this ceilMinutes(int|float $precision = 1) Ceil the current instance minute with given precision.
 * @method $this roundSecond(int|float $precision = 1, string $function = "round") Round the current instance second with given precision using the given function.
 * @method $this roundSeconds(int|float $precision = 1, string $function = "round") Round the current instance second with given precision using the given function.
 * @method $this floorSecond(int|float $precision = 1) Truncate the current instance second with given precision.
 * @method $this floorSeconds(int|float $precision = 1) Truncate the current instance second with given precision.
 * @method $this ceilSecond(int|float $precision = 1) Ceil the current instance second with given precision.
 * @method $this ceilSeconds(int|float $precision = 1) Ceil the current instance second with given precision.
 * @method $this roundMillennium(int|float $precision = 1, string $function = "round") Round the current instance millennium with given precision using the given function.
 * @method $this roundMillennia(int|float $precision = 1, string $function = "round") Round the current instance millennium with given precision using the given function.
 * @method $this floorMillennium(int|float $precision = 1) Truncate the current instance millennium with given precision.
 * @method $this floorMillennia(int|float $precision = 1) Truncate the current instance millennium with given precision.
 * @method $this ceilMillennium(int|float $precision = 1) Ceil the current instance millennium with given precision.
 * @method $this ceilMillennia(int|float $precision = 1) Ceil the current instance millennium with given precision.
 * @method $this roundCentury(int|float $precision = 1, string $function = "round") Round the current instance century with given precision using the given function.
 * @method $this roundCenturies(int|float $precision = 1, string $function = "round") Round the current instance century with given precision using the given function.
 * @method $this floorCentury(int|float $precision = 1) Truncate the current instance century with given precision.
 * @method $this floorCenturies(int|float $precision = 1) Truncate the current instance century with given precision.
 * @method $this ceilCentury(int|float $precision = 1) Ceil the current instance century with given precision.
 * @method $this ceilCenturies(int|float $precision = 1) Ceil the current instance century with given precision.
 * @method $this roundDecade(int|float $precision = 1, string $function = "round") Round the current instance decade with given precision using the given function.
 * @method $this roundDecades(int|float $precision = 1, string $function = "round") Round the current instance decade with given precision using the given function.
 * @method $this floorDecade(int|float $precision = 1) Truncate the current instance decade with given precision.
 * @method $this floorDecades(int|float $precision = 1) Truncate the current instance decade with given precision.
 * @method $this ceilDecade(int|float $precision = 1) Ceil the current instance decade with given precision.
 * @method $this ceilDecades(int|float $precision = 1) Ceil the current instance decade with given precision.
 * @method $this roundQuarter(int|float $precision = 1, string $function = "round") Round the current instance quarter with given precision using the given function.
 * @method $this roundQuarters(int|float $precision = 1, string $function = "round") Round the current instance quarter with given precision using the given function.
 * @method $this floorQuarter(int|float $precision = 1) Truncate the current instance quarter with given precision.
 * @method $this floorQuarters(int|float $precision = 1) Truncate the current instance quarter with given precision.
 * @method $this ceilQuarter(int|float $precision = 1) Ceil the current instance quarter with given precision.
 * @method $this ceilQuarters(int|float $precision = 1) Ceil the current instance quarter with given precision.
 * @method $this roundMillisecond(int|float $precision = 1, string $function = "round") Round the current instance millisecond with given precision using the given function.
 * @method $this roundMilliseconds(int|float $precision = 1, string $function = "round") Round the current instance millisecond with given precision using the given function.
 * @method $this floorMillisecond(int|float $precision = 1) Truncate the current instance millisecond with given precision.
 * @method $this floorMilliseconds(int|float $precision = 1) Truncate the current instance millisecond with given precision.
 * @method $this ceilMillisecond(int|float $precision = 1) Ceil the current instance millisecond with given precision.
 * @method $this ceilMilliseconds(int|float $precision = 1) Ceil the current instance millisecond with given precision.
 * @method $this roundMicrosecond(int|float $precision = 1, string $function = "round") Round the current instance microsecond with given precision using the given function.
 * @method $this roundMicroseconds(int|float $precision = 1, string $function = "round") Round the current instance microsecond with given precision using the given function.
 * @method $this floorMicrosecond(int|float $precision = 1) Truncate the current instance microsecond with given precision.
 * @method $this floorMicroseconds(int|float $precision = 1) Truncate the current instance microsecond with given precision.
 * @method $this ceilMicrosecond(int|float $precision = 1) Ceil the current instance microsecond with given precision.
 * @method $this ceilMicroseconds(int|float $precision = 1) Ceil the current instance microsecond with given precision.
 */
class CarbonInterval extends DateInterval implements CarbonConverterInterface, UnitValue
{
    use LocalFactory;
    use IntervalRounding;
    use IntervalStep;
    use MagicParameter;
    use Mixin {
        Mixin::mixin as baseMixin;
    }
    use Options;
    use ToStringFormat;

    /**
     * Unlimited parts for forHumans() method.
     *
     * INF constant can be used instead.
     */
    public const NO_LIMIT = -1;

    public const POSITIVE = 1;
    public const NEGATIVE = -1;

    /**
     * Interval spec period designators
     */
    public const PERIOD_PREFIX = 'P';
    public const PERIOD_YEARS = 'Y';
    public const PERIOD_MONTHS = 'M';
    public const PERIOD_DAYS = 'D';
    public const PERIOD_TIME_PREFIX = 'T';
    public const PERIOD_HOURS = 'H';
    public const PERIOD_MINUTES = 'M';
    public const PERIOD_SECONDS = 'S';

    public const SPECIAL_TRANSLATIONS = [
        1 => [
            'option' => CarbonInterface::ONE_DAY_WORDS,
            'future' => 'diff_tomorrow',
            'past' => 'diff_yesterday',
        ],
        2 => [
            'option' => CarbonInterface::TWO_DAY_WORDS,
            'future' => 'diff_after_tomorrow',
            'past' => 'diff_before_yesterday',
        ],
    ];

    protected static ?array $cascadeFactors = null;

    protected static array $formats = [
        'y' => 'y',
        'Y' => 'y',
        'o' => 'y',
        'm' => 'm',
        'n' => 'm',
        'W' => 'weeks',
        'd' => 'd',
        'j' => 'd',
        'z' => 'd',
        'h' => 'h',
        'g' => 'h',
        'H' => 'h',
        'G' => 'h',
        'i' => 'i',
        's' => 's',
        'u' => 'micro',
        'v' => 'milli',
    ];

    private static ?array $flipCascadeFactors = null;

    private static bool $floatSettersEnabled = false;

    /**
     * The registered macros.
     */
    protected static array $macros = [];

    /**
     * Timezone handler for settings() method.
     */
    protected DateTimeZone|string|int|null $timezoneSetting = null;

    /**
     * The input used to create the interval.
     */
    protected mixed $originalInput = null;

    /**
     * Start date if interval was created from a difference between 2 dates.
     */
    protected ?CarbonInterface $startDate = null;

    /**
     * End date if interval was created from a difference between 2 dates.
     */
    protected ?CarbonInterface $endDate = null;

    /**
     * End date if interval was created from a difference between 2 dates.
     */
    protected ?DateInterval $rawInterval = null;

    /**
     * Flag if the interval was made from a diff with absolute flag on.
     */
    protected bool $absolute = false;

    protected ?array $initialValues = null;

    /**
     * Set the instance's timezone from a string or object.
     */
    public function setTimezone(DateTimeZone|string|int $timezone): static
    {}

    /**
     * Set the instance's timezone from a string or object and add/subtract the offset difference.
     */
    public function shiftTimezone(DateTimeZone|string|int $timezone): static
    {}

    /**
     * Mapping of units and factors for cascading.
     *
     * Should only be modified by changing the factors or referenced constants.
     */
    public static function getCascadeFactors(): array
    {}

    protected static function getDefaultCascadeFactors(): array
    {}

    /**
     * Set default cascading factors for ->cascade() method.
     *
     * @param array $cascadeFactors
     */
    public static function setCascadeFactors(array $cascadeFactors)
    {}

    /**
     * This option allow you to opt-in for the Carbon 3 behavior where float
     * values will no longer be cast to integer (so truncated).
     *
     * ⚠️ This settings will be applied globally, which mean your whole application
     * code including the third-party dependencies that also may use Carbon will
     * adopt the new behavior.
     */
    public static function enableFloatSetters(bool $floatSettersEnabled = true): void
    {}

    ///////////////////////////////////////////////////////////////////
    //////////////////////////// CONSTRUCTORS /////////////////////////
    ///////////////////////////////////////////////////////////////////

    /**
     * Create a new CarbonInterval instance.
     *
     * @param Closure|DateInterval|Duration|string|int|null $years
     * @param int|float|null                                $months
     * @param int|float|null                                $weeks
     * @param int|float|null                                $days
     * @param int|float|null                                $hours
     * @param int|float|null                                $minutes
     * @param int|float|null                                $seconds
     * @param int|float|null                                $microseconds
     *
     * @throws Exception when the interval_spec (passed as $years) cannot be parsed as an interval.
     */
    public function __construct($years = null, $months = null, $weeks = null, $days = null, $hours = null, $minutes = null, $seconds = null, $microseconds = null, $inverted = false)
    { func_get_args(); }

    /**
     * Returns the factor for a given source-to-target couple.
     *
     * @param string $source
     * @param string $target
     *
     * @return int|float|null
     */
    public static function getFactor($source, $target)
    {}

    /**
     * Returns the factor for a given source-to-target couple if set,
     * else try to find the appropriate constant as the factor, such as Carbon::DAYS_PER_WEEK.
     *
     * @param string $source
     * @param string $target
     *
     * @return int|float|null
     */
    public static function getFactorWithDefault($source, $target)
    {}

    /**
     * Returns current config for days per week.
     *
     * @return int|float
     */
    public static function getDaysPerWeek()
    {}

    /**
     * Returns current config for hours per day.
     *
     * @return int|float
     */
    public static function getHoursPerDay()
    {}

    /**
     * Returns current config for minutes per hour.
     *
     * @return int|float
     */
    public static function getMinutesPerHour()
    {}

    /**
     * Returns current config for seconds per minute.
     *
     * @return int|float
     */
    public static function getSecondsPerMinute()
    {}

    /**
     * Returns current config for microseconds per second.
     *
     * @return int|float
     */
    public static function getMillisecondsPerSecond()
    {}

    /**
     * Returns current config for microseconds per second.
     *
     * @return int|float
     */
    public static function getMicrosecondsPerMillisecond()
    {}

    /**
     * Create a new CarbonInterval instance from specific values.
     * This is an alias for the constructor that allows better fluent
     * syntax as it allows you to do CarbonInterval::create(1)->fn() rather than
     * (new CarbonInterval(1))->fn().
     *
     * @param int $years
     * @param int $months
     * @param int $weeks
     * @param int $days
     * @param int $hours
     * @param int $minutes
     * @param int $seconds
     * @param int $microseconds
     *
     * @throws Exception when the interval_spec (passed as $years) cannot be parsed as an interval.
     *
     * @return static
     */
    public static function create($years = null, $months = null, $weeks = null, $days = null, $hours = null, $minutes = null, $seconds = null, $microseconds = null, $inverted = false)
    {}

    /**
     * Parse a string into a new CarbonInterval object according to the specified format.
     *
     * @example
     * ```
     * echo Carboninterval::createFromFormat('H:i', '1:30');
     * ```
     *
     * @param string      $format   Format of the $interval input string
     * @param string|null $interval Input string to convert into an interval
     *
     * @throws \Carbon\Exceptions\ParseErrorException when the $interval cannot be parsed as an interval.
     *
     * @return static
     */
    public static function createFromFormat(string $format, ?string $interval): static
    {}

    public static function monthWithAnchorDay(int $day): static
    {}

    public static function monthNoOverflow(): static
    {}

    public static function yearWithAnchorDay(int $day): static
    {}

    public static function yearNoOverflow(): static
    {}

    /**
     * Return the original source used to create the current interval.
     *
     * @return array|int|string|Closure|DateInterval|Duration|mixed|null
     */
    public function original()
    {}

    /**
     * Return the start date if interval was created from a difference between 2 dates.
     *
     * @return CarbonInterface|null
     */
    public function start(): ?CarbonInterface
    {}

    /**
     * Return the end date if interval was created from a difference between 2 dates.
     *
     * @return CarbonInterface|null
     */
    public function end(): ?CarbonInterface
    {}

    /**
     * Get rid of the original input, start date and end date that may be kept in memory.
     *
     * @return $this
     */
    public function optimize(): static
    {}

    /**
     * Get a copy of the instance.
     *
     * @return static
     */
    public function copy(): static
    {}

    /**
     * Get a copy of the instance.
     *
     * @return static
     */
    public function clone(): static
    {}

    /**
     * Provide static helpers to create instances.  Allows CarbonInterval::years(3).
     *
     * Note: This is done using the magic method to allow static and instance methods to
     *       have the same names.
     *
     * @param string $method     magic method name called
     * @param array  $parameters parameters list
     *
     * @return static|mixed|null
     */
    public static function __callStatic(string $method, array $parameters)
    {}

    /**
     * Evaluate the PHP generated by var_export() and recreate the exported CarbonInterval instance.
     *
     * @param array $dump data as exported by var_export()
     *
     * @return static
     */
    #[ReturnTypeWillChange]
    public static function __set_state($dump)
    {}

    /**
     * Return the current context from inside a macro callee or a new one if static.
     *
     * @return static
     */
    protected static function this(): static
    {}

    /**
     * Creates a CarbonInterval from string.
     *
     * Format:
     *
     * Suffix | Unit    | Example | DateInterval expression
     * -------|---------|---------|------------------------
     * y      | years   |   1y    | P1Y
     * mo     | months  |   3mo   | P3M
     * w      | weeks   |   2w    | P2W
     * d      | days    |  28d    | P28D
     * h      | hours   |   4h    | PT4H
     * m      | minutes |  12m    | PT12M
     * s      | seconds |  59s    | PT59S
     *
     * e. g. `1w 3d 4h 32m 23s` is converted to 10 days 4 hours 32 minutes and 23 seconds.
     *
     * Special cases:
     *  - An empty string will return a zero interval
     *  - Fractions are allowed for weeks, days, hours and minutes and will be converted
     *    and rounded to the next smaller value (caution: 0.5w = 4d)
     *
     * @param string $intervalDefinition
     *
     * @throws InvalidIntervalException
     *
     * @return static
     */
    public static function fromString(string $intervalDefinition): static
    {}

    /**
     * Creates a CarbonInterval from string using a different locale.
     *
     * @param string      $interval interval string in the given language (may also contain English).
     * @param string|null $locale   if locale is null or not specified, current global locale will be used instead.
     *
     * @return static
     */
    public static function parseFromLocale(string $interval, ?string $locale = null): static
    {}

    /**
     * Create an interval from the difference between 2 dates.
     *
     * @param \Carbon\Carbon|\DateTimeInterface|mixed $start
     * @param \Carbon\Carbon|\DateTimeInterface|mixed $end
     *
     * @return static
     */
    public static function diff($start, $end = null, bool $absolute = false, array $skip = []): static
    {}

    /**
     * Invert the interval if it's inverted.
     *
     * @param bool $absolute do nothing if set to false
     *
     * @return $this
     */
    public function abs(bool $absolute = false): static
    {}

    /**
     * @alias abs
     *
     * Invert the interval if it's inverted.
     *
     * @param bool $absolute do nothing if set to false
     *
     * @return $this
     */
    public function absolute(bool $absolute = true): static
    {}

    /**
     * Cast the current instance into the given class.
     *
     * @template T of DateInterval|Duration
     *
     * @psalm-param class-string<T> $className The $className::instance() method will be called to cast the current object.
     *
     * @return T
     */
    public function cast(string $className): mixed
    {}

    /**
     * Create a CarbonInterval instance from a DateInterval one.  Can not instance
     * DateInterval objects created from DateTime::diff() as you can't externally
     * set the $days field.
     *
     * @param DateInterval|Duration $interval
     * @param bool                  $skipCopy set to true to return the passed object
     *                                        (without copying it) if it's already of the
     *                                        current class
     *
     * @return static
     */
    public static function instance(DateInterval|Duration $interval, array $skip = [], bool $skipCopy = false): static
    {}

    /**
     * Make a CarbonInterval instance from given variable if possible.
     *
     * Always return a new instance. Parse only strings and only these likely to be intervals (skip dates
     * and recurrences). Throw an exception for invalid format, but otherwise return null.
     *
     * @param DateInterval|Closure|Unit|Duration|mixed|int|string|null $interval interval or number of the given $unit
     * @param Unit|string|null                                         $unit     if specified, $interval must be an integer
     * @param bool                                                     $skipCopy set to true to return the passed object
     *                                                                           (without copying it) if it's already of the
     *                                                                           current class
     *
     * @return static|null
     */
    public static function make($interval, $unit = null, bool $skipCopy = false): ?self
    {}

    protected static function makeFromString(string $interval): ?self
    {}

    protected function resolveInterval($interval): ?self
    {}

    /**
     * Sets up a DateInterval from the relative parts of the string.
     *
     * @param string $datetime
     *
     * @return static
     *
     * @link https://php.net/manual/en/dateinterval.createfromdatestring.php
     */
    public static function createFromDateString(string $datetime): static
    {}

    ///////////////////////////////////////////////////////////////////
    ///////////////////////// GETTERS AND SETTERS /////////////////////
    ///////////////////////////////////////////////////////////////////

    /**
     * Get a part of the CarbonInterval object.
     */
    public function get(Unit|string $name): int|float|string|null
    {}

    /**
     * Get a part of the CarbonInterval object.
     */
    public function __get(string $name): int|float|string|null
    {}

    /**
     * Set a part of the CarbonInterval object.
     *
     * @param Unit|string|array $name
     * @param int               $value
     *
     * @throws UnknownSetterException
     *
     * @return $this
     */
    public function set($name, $value = null): static
    {}

    /**
     * Set a part of the CarbonInterval object.
     *
     * @param string $name
     * @param int    $value
     *
     * @throws UnknownSetterException
     */
    public function __set(string $name, $value)
    {}

    /**
     * Allow setting of weeks and days to be cumulative.
     *
     * @param int $weeks Number of weeks to set
     * @param int $days  Number of days to set
     *
     * @return static
     */
    public function weeksAndDays(int $weeks, int $days): static
    {}

    /**
     * Returns true if the interval is empty for each unit.
     *
     * @return bool
     */
    public function isEmpty(): bool
    {}

    /**
     * Register a custom macro.
     *
     * Pass null macro to remove it.
     *
     * @example
     * ```
     * CarbonInterval::macro('twice', function () {
     *   return $this->times(2);
     * });
     * echo CarbonInterval::hours(2)->twice();
     * ```
     *
     * @param-closure-this static $macro
     */
    public static function macro(string $name, ?callable $macro): void
    {}

    /**
     * Register macros from a mixin object.
     *
     * @example
     * ```
     * CarbonInterval::mixin(new class {
     *   public function daysToHours() {
     *     return function () {
     *       $this->hours += $this->days;
     *       $this->days = 0;
     *
     *       return $this;
     *     };
     *   }
     *   public function hoursToDays() {
     *     return function () {
     *       $this->days += $this->hours;
     *       $this->hours = 0;
     *
     *       return $this;
     *     };
     *   }
     * });
     * echo CarbonInterval::hours(5)->hoursToDays() . "\n";
     * echo CarbonInterval::days(5)->daysToHours() . "\n";
     * ```
     *
     * @param object|string $mixin
     *
     * @throws ReflectionException
     *
     * @return void
     */
    public static function mixin($mixin): void
    {}

    /**
     * Check if macro is registered.
     *
     * @param string $name
     *
     * @return bool
     */
    public static function hasMacro(string $name): bool
    {}

    /**
     * Call given macro.
     *
     * @param string $name
     * @param array  $parameters
     *
     * @return mixed
     */
    protected function callMacro(string $name, array $parameters)
    {}

    /**
     * Allow fluent calls on the setters... CarbonInterval::years(3)->months(5)->day().
     *
     * Note: This is done using the magic method to allow static and instance methods to
     *       have the same names.
     *
     * @param string $method     magic method name called
     * @param array  $parameters parameters list
     *
     * @throws BadFluentSetterException|Throwable
     *
     * @return static|int|float|string
     */
    public function __call(string $method, array $parameters)
    {}

    protected function getForHumansInitialVariables($syntax, $short): array
    {}

    /**
     * @param mixed $syntax
     * @param mixed $short
     * @param mixed $parts
     * @param mixed $options
     *
     * @return array
     */
    protected function getForHumansParameters($syntax = null, $short = false, $parts = self::NO_LIMIT, $options = null): array
    {}

    protected static function getRoundingMethodFromOptions(int $options): ?string
    {}

    /**
     * Returns interval values as an array where key are the unit names and values the counts.
     *
     * @return int[]
     */
    public function toArray(): array
    {}

    /**
     * Returns interval non-zero values as an array where key are the unit names and values the counts.
     *
     * @return int[]
     */
    public function getNonZeroValues(): array
    {}

    /**
     * Returns interval values as an array where key are the unit names and values the counts
     * from the biggest non-zero one the the smallest non-zero one.
     *
     * @return int[]
     */
    public function getValuesSequence(): array
    {}

    /**
     * Get the current interval in a human readable format in the current locale.
     *
     * @example
     * ```
     * echo CarbonInterval::fromString('4d 3h 40m')->forHumans() . "\n";
     * echo CarbonInterval::fromString('4d 3h 40m')->forHumans(['parts' => 2]) . "\n";
     * echo CarbonInterval::fromString('4d 3h 40m')->forHumans(['parts' => 3, 'join' => true]) . "\n";
     * echo CarbonInterval::fromString('4d 3h 40m')->forHumans(['short' => true]) . "\n";
     * echo CarbonInterval::fromString('1d 24h')->forHumans(['join' => ' or ']) . "\n";
     * echo CarbonInterval::fromString('1d 24h')->forHumans(['minimumUnit' => 'hour']) . "\n";
     * ```
     *
     * @param int|array $syntax  if array passed, parameters will be extracted from it, the array may contain:
     *                           ⦿ 'syntax' entry (see below)
     *                           ⦿ 'short' entry (see below)
     *                           ⦿ 'parts' entry (see below)
     *                           ⦿ 'options' entry (see below)
     *                           ⦿ 'skip' entry, list of units to skip (array of strings or a single string,
     *                           ` it can be the unit name (singular or plural) or its shortcut
     *                           ` (y, m, w, d, h, min, s, ms, µs).
     *                           ⦿ 'aUnit' entry, prefer "an hour" over "1 hour" if true
     *                           ⦿ 'altNumbers' entry, use alternative numbers if available
     *                           ` (from the current language if true is passed, from the given language(s)
     *                           ` if array or string is passed)
     *                           ⦿ 'join' entry determines how to join multiple parts of the string
     *                           `  - if $join is a string, it's used as a joiner glue
     *                           `  - if $join is a callable/closure, it get the list of string and should return a string
     *                           `  - if $join is an array, the first item will be the default glue, and the second item
     *                           `    will be used instead of the glue for the last item
     *                           `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                           `  - if $join is missing, a space will be used as glue
     *                           ⦿ 'minimumUnit' entry determines the smallest unit of time to display can be long or
     *                           `  short form of the units, e.g. 'hour' or 'h' (default value: s)
     *                           ⦿ 'locale' language in which the diff should be output (has no effect if 'translator' key is set)
     *                           ⦿ 'translator' a custom translator to use to translator the output.
     *                           if int passed, it adds modifiers:
     *                           Possible values:
     *                           - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                           - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                           - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                           Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool      $short   displays short format of time units
     * @param int       $parts   maximum number of parts to display (default value: -1: no limits)
     * @param int       $options human diff options
     *
     * @throws Exception
     *
     * @return string
     */
    public function forHumans($syntax = null, $short = false, $parts = self::NO_LIMIT, $options = null): string
    {}

    public function format(string $format): string
    {}

    /**
     * Format the instance as a string using the forHumans() function.
     *
     * @throws Exception
     *
     * @return string
     */
    public function __toString(): string
    {}

    /**
     * Return native DateInterval PHP object matching the current instance.
     *
     * @example
     * ```
     * var_dump(CarbonInterval::hours(2)->toDateInterval());
     * ```
     *
     * @return DateInterval
     */
    public function toDateInterval(): DateInterval
    {}

    /**
     * Convert the current instance to a native {@link Duration} object (PHP >= 8.6).
     *
     * Only the time part (hours, minutes, seconds and fractions of second) can be
     * represented as a "stop-watch" duration. Nanoseconds below the microsecond
     * precision of CarbonInterval are truncated. An InvalidIntervalException is
     * thrown if the interval bears any date unit (years, months, weeks or days)
     * because such units cannot be unambiguously converted to elapsed time.
     *
     * @example
     * ```
     * var_dump(CarbonInterval::hours(2)->toDuration());
     * ```
     *
     * @throws InvalidIntervalException when the interval contains date units
     *
     * @return Duration
     */
    public function toDuration(): Duration
    {}

    /**
     * Create a CarbonInterval instance from a native {@link Duration} object (PHP >= 8.6).
     *
     * Nanoseconds below the microsecond precision of CarbonInterval are truncated.
     * A negative duration produces an interval with negative time components.
     *
     * @example
     * ```
     * var_dump(CarbonInterval::fromDuration(Time\Duration::fromMinutes(5)));
     * ```
     *
     * @param Duration $duration
     *
     * @return static
     */
    public static function fromDuration(Duration $duration): static
    {}

    /**
     * Convert the interval to a CarbonPeriod.
     *
     * @param DateTimeInterface|string|int ...$params Start date, [end date or recurrences] and optional settings.
     *
     * @return CarbonPeriod
     */
    public function toPeriod(...$params): CarbonPeriod
    {}

    /**
     * Decompose the current interval into
     *
     * @param DateInterval|Duration|Closure|Unit|mixed|int|string|null $interval interval or number of the given $unit
     * @param Unit|string|null                                         $unit     if specified, $interval must be an integer
     *
     * @return CarbonPeriod
     */
    public function stepBy($interval, Unit|string|null $unit = null): CarbonPeriod
    {}

    /**
     * Invert the interval.
     *
     * @param bool|int $inverted if a parameter is passed, the passed value cast as 1 or 0 is used
     *                           as the new value of the ->invert property.
     *
     * @return $this
     */
    public function invert($inverted = null): static
    { func_get_args(); }

    protected function solveNegativeInterval(): static
    {}

    /**
     * Add the passed interval to the current instance.
     *
     * @param Unit|DateInterval|Duration|string $unit
     * @param int|float                         $value
     *
     * @return $this
     */
    public function add($unit, $value = 1): static
    {}

    /**
     * Subtract the passed interval to the current instance.
     *
     * @param Unit|DateInterval|Duration|string $unit
     * @param int|float                         $value
     *
     * @return $this
     */
    public function sub($unit, $value = 1): static
    {}

    /**
     * Subtract the passed interval to the current instance.
     *
     * @param Unit|DateInterval|Duration|string $unit
     * @param int|float                         $value
     *
     * @return $this
     */
    public function subtract($unit, $value = 1): static
    {}

    /**
     * Add given parameters to the current interval.
     *
     * @param int       $years
     * @param int       $months
     * @param int|float $weeks
     * @param int|float $days
     * @param int|float $hours
     * @param int|float $minutes
     * @param int|float $seconds
     * @param int|float $microseconds
     *
     * @return $this
     */
    public function plus(
        $years = 0,
        $months = 0,
        $weeks = 0,
        $days = 0,
        $hours = 0,
        $minutes = 0,
        $seconds = 0,
        $microseconds = 0
    ): static {}

    /**
     * Subtract given parameters to the current interval.
     *
     * @param int       $years
     * @param int       $months
     * @param int|float $weeks
     * @param int|float $days
     * @param int|float $hours
     * @param int|float $minutes
     * @param int|float $seconds
     * @param int|float $microseconds
     *
     * @return $this
     */
    public function minus(
        $years = 0,
        $months = 0,
        $weeks = 0,
        $days = 0,
        $hours = 0,
        $minutes = 0,
        $seconds = 0,
        $microseconds = 0
    ): static {}

    /**
     * Multiply current instance given number of times. times() is naive, it multiplies each unit
     * (so day can be greater than 31, hour can be greater than 23, etc.) and the result is rounded
     * separately for each unit.
     *
     * Use times() when you want a fast and approximated calculation that does not cascade units.
     *
     * For a precise and cascaded calculation,
     *
     * @see multiply()
     *
     * @param float|int $factor
     *
     * @return $this
     */
    public function times($factor): static
    {}

    /**
     * Divide current instance by a given divider. shares() is naive, it divides each unit separately
     * and the result is rounded for each unit. So 5 hours and 20 minutes shared by 3 becomes 2 hours
     * and 7 minutes.
     *
     * Use shares() when you want a fast and approximated calculation that does not cascade units.
     *
     * For a precise and cascaded calculation,
     *
     * @see divide()
     *
     * @param float|int $divider
     *
     * @return $this
     */
    public function shares($divider): static
    {}

    protected function copyProperties(self $interval, $ignoreSign = false): static
    {}

    /**
     * Multiply and cascade current instance by a given factor.
     *
     * @param float|int $factor
     *
     * @return $this
     */
    public function multiply($factor): static
    {}

    /**
     * Divide and cascade current instance by a given divider.
     *
     * @param float|int $divider
     *
     * @return $this
     */
    public function divide($divider): static
    {}

    /**
     * Get the interval_spec string of a date interval.
     *
     * @param DateInterval $interval
     *
     * @return string
     */
    public static function getDateIntervalSpec(
        DateInterval $interval,
        bool $microseconds = false,
        array $skip = [],
        bool $withNegatives = false,
    ): string {}

    /**
     * Get the interval_spec string.
     *
     * @return string
     */
    public function spec(bool $microseconds = false, bool $withNegatives = false): string
    {}

    /**
     * Comparing 2 date intervals.
     *
     * @param DateInterval $first
     * @param DateInterval $second
     *
     * @return int 0, 1 or -1
     */
    public static function compareDateIntervals(DateInterval $first, DateInterval $second): int
    {}

    /**
     * Comparing with passed interval.
     *
     * @param DateInterval $interval
     *
     * @return int 0, 1 or -1
     */
    public function compare(DateInterval $interval): int
    {}

    /**
     * Convert overflowed values into bigger units.
     *
     * @return $this
     */
    public function cascade(): static
    {}

    public function hasNegativeValues(): bool
    {}

    public function hasPositiveValues(): bool
    {}

    /**
     * Get amount of given unit equivalent to the interval.
     *
     * @param string $unit
     *
     * @throws UnknownUnitException|UnitNotConfiguredException
     *
     * @return float
     */
    public function total(string $unit): float
    {}

    /**
     * Determines if the instance is equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @see equalTo()
     *
     * @return bool
     */
    public function eq($interval): bool
    {}

    /**
     * Determines if the instance is equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @return bool
     */
    public function equalTo($interval): bool
    {}

    /**
     * Determines if the instance is not equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @see notEqualTo()
     *
     * @return bool
     */
    public function ne($interval): bool
    {}

    /**
     * Determines if the instance is not equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @return bool
     */
    public function notEqualTo($interval): bool
    {}

    /**
     * Determines if the instance is greater (longer) than another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @see greaterThan()
     *
     * @return bool
     */
    public function gt($interval): bool
    {}

    /**
     * Determines if the instance is greater (longer) than another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @return bool
     */
    public function greaterThan($interval): bool
    {}

    /**
     * Determines if the instance is greater (longer) than or equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @see greaterThanOrEqualTo()
     *
     * @return bool
     */
    public function gte($interval): bool
    {}

    /**
     * Determines if the instance is greater (longer) than or equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @return bool
     */
    public function greaterThanOrEqualTo($interval): bool
    {}

    /**
     * Determines if the instance is less (shorter) than another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @see lessThan()
     *
     * @return bool
     */
    public function lt($interval): bool
    {}

    /**
     * Determines if the instance is less (shorter) than another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @return bool
     */
    public function lessThan($interval): bool
    {}

    /**
     * Determines if the instance is less (shorter) than or equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @see lessThanOrEqualTo()
     *
     * @return bool
     */
    public function lte($interval): bool
    {}

    /**
     * Determines if the instance is less (shorter) than or equal to another
     *
     * @param CarbonInterval|DateInterval|mixed $interval
     *
     * @return bool
     */
    public function lessThanOrEqualTo($interval): bool
    {}

    /**
     * Determines if the instance is between two others.
     *
     * The third argument allow you to specify if bounds are included or not (true by default)
     * but for when you including/excluding bounds may produce different results in your application,
     * we recommend to use the explicit methods ->betweenIncluded() or ->betweenExcluded() instead.
     *
     * @example
     * ```
     * CarbonInterval::hours(48)->between(CarbonInterval::day(), CarbonInterval::days(3)); // true
     * CarbonInterval::hours(48)->between(CarbonInterval::day(), CarbonInterval::hours(36)); // false
     * CarbonInterval::hours(48)->between(CarbonInterval::day(), CarbonInterval::days(2)); // true
     * CarbonInterval::hours(48)->between(CarbonInterval::day(), CarbonInterval::days(2), false); // false
     * ```
     *
     * @param CarbonInterval|DateInterval|mixed $interval1
     * @param CarbonInterval|DateInterval|mixed $interval2
     * @param bool                              $equal     Indicates if an equal to comparison should be done
     *
     * @return bool
     */
    public function between($interval1, $interval2, bool $equal = true): bool
    {}

    /**
     * Determines if the instance is between two others, bounds excluded.
     *
     * @example
     * ```
     * CarbonInterval::hours(48)->betweenExcluded(CarbonInterval::day(), CarbonInterval::days(3)); // true
     * CarbonInterval::hours(48)->betweenExcluded(CarbonInterval::day(), CarbonInterval::hours(36)); // false
     * CarbonInterval::hours(48)->betweenExcluded(CarbonInterval::day(), CarbonInterval::days(2)); // true
     * ```
     *
     * @param CarbonInterval|DateInterval|mixed $interval1
     * @param CarbonInterval|DateInterval|mixed $interval2
     *
     * @return bool
     */
    public function betweenIncluded($interval1, $interval2): bool
    {}

    /**
     * Determines if the instance is between two others, bounds excluded.
     *
     * @example
     * ```
     * CarbonInterval::hours(48)->betweenExcluded(CarbonInterval::day(), CarbonInterval::days(3)); // true
     * CarbonInterval::hours(48)->betweenExcluded(CarbonInterval::day(), CarbonInterval::hours(36)); // false
     * CarbonInterval::hours(48)->betweenExcluded(CarbonInterval::day(), CarbonInterval::days(2)); // false
     * ```
     *
     * @param CarbonInterval|DateInterval|mixed $interval1
     * @param CarbonInterval|DateInterval|mixed $interval2
     *
     * @return bool
     */
    public function betweenExcluded($interval1, $interval2): bool
    {}

    /**
     * Determines if the instance is between two others
     *
     * @example
     * ```
     * CarbonInterval::hours(48)->isBetween(CarbonInterval::day(), CarbonInterval::days(3)); // true
     * CarbonInterval::hours(48)->isBetween(CarbonInterval::day(), CarbonInterval::hours(36)); // false
     * CarbonInterval::hours(48)->isBetween(CarbonInterval::day(), CarbonInterval::days(2)); // true
     * CarbonInterval::hours(48)->isBetween(CarbonInterval::day(), CarbonInterval::days(2), false); // false
     * ```
     *
     * @param CarbonInterval|DateInterval|mixed $interval1
     * @param CarbonInterval|DateInterval|mixed $interval2
     * @param bool                              $equal     Indicates if an equal to comparison should be done
     *
     * @return bool
     */
    public function isBetween($interval1, $interval2, bool $equal = true): bool
    {}

    /**
     * Round the current instance at the given unit with given precision if specified and the given function.
     *
     * @throws Exception
     */
    public function roundUnit(string $unit, DateInterval|Duration|string|int|float $precision = 1, string $function = 'round'): static
    {}

    /**
     * Truncate the current instance at the given unit with given precision if specified.
     *
     * @param string                                      $unit
     * @param float|int|string|DateInterval|Duration|null $precision
     *
     * @throws Exception
     *
     * @return $this
     */
    public function floorUnit(string $unit, $precision = 1): static
    {}

    /**
     * Ceil the current instance at the given unit with given precision if specified.
     *
     * @param string                                      $unit
     * @param float|int|string|DateInterval|Duration|null $precision
     *
     * @throws Exception
     *
     * @return $this
     */
    public function ceilUnit(string $unit, $precision = 1): static
    {}

    /**
     * Round the current instance second with given precision if specified.
     *
     * @param float|int|string|DateInterval|Duration|null $precision
     * @param string                                      $function
     *
     * @throws Exception
     *
     * @return $this
     */
    public function round($precision = 1, string $function = 'round'): static
    {}

    /**
     * Round the current instance second with given precision if specified.
     *
     * @throws Exception
     *
     * @return $this
     */
    public function floor(DateInterval|Duration|string|float|int $precision = 1): static
    {}

    /**
     * Ceil the current instance second with given precision if specified.
     *
     * @throws Exception
     *
     * @return $this
     */
    public function ceil(DateInterval|Duration|string|float|int $precision = 1): static
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * @template T
     *
     * @param T     $interval
     * @param mixed $original
     *
     * @return T
     */
    private static function withOriginal(mixed $interval, mixed $original): mixed
    {}

    private static function standardizeUnit(string $unit): string
    {}

    private static function getFlipCascadeFactors(): array
    {}

    /**
     * @template T of DateInterval|Duration
     *
     * @param DateInterval|Duration $interval
     *
     * @psalm-param class-string<T> $className
     *
     * @return T
     */
    private static function castIntervalToClass(DateInterval|Duration $interval, string $className, array $skip = []): DateInterval|Duration
    {}

    /**
     * @template T of DateInterval
     *
     * @param DateInterval $interval
     *
     * @psalm-param class-string<T> $className
     *
     * @return T
     */
    private static function buildInstance(
        DateInterval $interval,
        string $className,
        array $skip = [],
    ): object {}

    /**
     * As demonstrated by rlanvin (https://github.com/rlanvin) in
     * https://github.com/briannesbitt/Carbon/issues/3018#issuecomment-2888538438
     *
     * Modifying the output of serialize() to change the class name and unserializing
     * the tweaked string allows creating new interval instances where the ->days
     * property can be set. It's not possible neither with `new` nto with `__set_state`.
     *
     * It has a non-negligible performance cost, so we'll use this method only if
     * $interval->days !== false.
     */
    private static function buildSerializationString(
        DateInterval $interval,
        string $className,
        array $skip = [],
    ): ?string {}

    private static function copyStep(self $from, self $to): void
    {}

    private static function copyNegativeUnits(DateInterval $from, DateInterval $to): void
    {}

    private function invertCascade(array $values): static
    {}

    private function doCascade(bool $deep): static
    {}

    private function needsDeclension(string $mode, int $index, int $parts): bool
    {}

    private function checkIntegerValue(string $name, mixed $value): void
    {}

    /**
     * Throw an exception if precision loss when storing the given value as an integer would be >= 1.0.
     */
    private function assertSafeForInteger(string $name, mixed $value): void
    {}

    private function handleDecimalPart(string $unit, mixed $value, mixed $integerValue): void
    {}

    private function getInnerValues(): array
    {}

    private function checkStartAndEnd(): void
    {}

    /** @return $this */
    private function setSetting(string $setting, mixed $value): self
    {}

    private static function carbonOrMake(mixed $dateTime): CarbonInterface
    {}

    private static function incrementUnit(DateInterval $instance, string $unit, int $value): void
    {}

    /** @codeCoverageIgnore */
    private static function setIntervalUnit(DateInterval $instance, string $unit, mixed $value): void
    {}
}
