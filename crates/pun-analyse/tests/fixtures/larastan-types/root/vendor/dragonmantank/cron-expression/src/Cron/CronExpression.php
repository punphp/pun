<?php

declare(strict_types=1);

namespace Cron;

use DateTime;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Exception;
use InvalidArgumentException;
use LogicException;
use RuntimeException;

/**
 * CRON expression parser that can determine whether or not a CRON expression is
 * due to run, the next run date and previous run date of a CRON expression.
 * The determinations made by this class are accurate if checked run once per
 * minute (seconds are dropped from date time comparisons).
 *
 * Schedule parts must map to:
 * minute [0-59], hour [0-23], day of month, month [1-12|JAN-DEC], day of week
 * [1-7|MON-SUN], and an optional year.
 *
 * @see http://en.wikipedia.org/wiki/Cron
 */
class CronExpression
{
    public const MINUTE = 0;
    public const HOUR = 1;
    public const DAY = 2;
    public const MONTH = 3;
    public const WEEKDAY = 4;

    /** @deprecated */
    public const YEAR = 5;

    public const MAPPINGS = [
        '@yearly' => '0 0 1 1 *',
        '@annually' => '0 0 1 1 *',
        '@monthly' => '0 0 1 * *',
        '@weekly' => '0 0 * * 0',
        '@daily' => '0 0 * * *',
        '@midnight' => '0 0 * * *',
        '@hourly' => '0 * * * *',
    ];

    /**
     * @var array<int, string> CRON expression parts
     */
    protected $cronParts;

    /**
     * @var FieldFactoryInterface CRON field factory
     */
    protected $fieldFactory;

    /**
     * @var int Max iteration count when searching for next run date
     */
    protected $maxIterationCount = 1000;

    /**
     * @var array<int, int> Order in which to test of cron parts
     */
    protected static $order = [
        self::YEAR,
        self::MONTH,
        self::DAY,
        self::WEEKDAY,
        self::HOUR,
        self::MINUTE,
    ];

    /**
     * @var array<string, string>
     */
    private static $registeredAliases = self::MAPPINGS;

    /**
     * Registered a user defined CRON Expression Alias.
     *
     * @throws LogicException If the expression or the alias name are invalid
     *                         or if the alias is already registered.
     */
    public static function registerAlias(string $alias, string $expression): void
    {}

    /**
     * Unregistered a user defined CRON Expression Alias.
     *
     * @throws LogicException If the user tries to unregister a built-in alias
     */
    public static function unregisterAlias(string $alias): bool
    {}

    /**
     * Tells whether a CRON Expression alias is registered.
     */
    public static function supportsAlias(string $alias): bool
    {}

    /**
     * Returns all registered aliases as an associated array where the aliases are the key
     * and their associated expressions are the values.
     *
     * @return array<string, string>
     */
    public static function getAliases(): array
    {}

    /**
     * @deprecated since version 3.0.2, use __construct instead.
     */
    public static function factory(string $expression, ?FieldFactoryInterface $fieldFactory = null): CronExpression
    {}

    /**
     * Validate a CronExpression.
     *
     * @param string $expression the CRON expression to validate
     *
     * @return bool True if a valid CRON expression was passed. False if not.
     */
    public static function isValidExpression(string $expression): bool
    {}

    /**
     * Parse a CRON expression.
     *
     * @param string $expression CRON expression (e.g. '8 * * * *')
     * @param null|FieldFactoryInterface $fieldFactory Factory to create cron fields
     * @throws InvalidArgumentException
     */
    public function __construct(string $expression, ?FieldFactoryInterface $fieldFactory = null)
    {}

    /**
     * Set or change the CRON expression.
     *
     * @param string $value CRON expression (e.g. 8 * * * *)
     *
     * @throws \InvalidArgumentException if not a valid CRON expression
     *
     * @return CronExpression
     */
    public function setExpression(string $value): CronExpression
    {}

    /**
     * Set part of the CRON expression.
     *
     * @param int $position The position of the CRON expression to set
     * @param string $value The value to set
     *
     * @throws \InvalidArgumentException if the value is not valid for the part
     *
     * @return CronExpression
     */
    public function setPart(int $position, string $value): CronExpression
    {}

    /**
     * Set max iteration count for searching next run dates.
     *
     * @param int $maxIterationCount Max iteration count when searching for next run date
     *
     * @return CronExpression
     */
    public function setMaxIterationCount(int $maxIterationCount): CronExpression
    {}

    /**
     * Get a next run date relative to the current date or a specific date
     *
     * @param string|\DateTimeInterface $currentTime      Relative calculation date
     * @param int                       $nth              Number of matches to skip before returning a
     *                                                    matching next run date.  0, the default, will return the
     *                                                    current date and time if the next run date falls on the
     *                                                    current date and time.  Setting this value to 1 will
     *                                                    skip the first match and go to the second match.
     *                                                    Setting this value to 2 will skip the first 2
     *                                                    matches and so on.
     * @param bool                      $allowCurrentDate Set to TRUE to return the current date if
     *                                                    it matches the cron expression.
     * @param null|string               $timeZone         TimeZone to use instead of the system default
     *
     * @throws \RuntimeException on too many iterations
     * @throws \Exception
     *
     * @return \DateTime
     */
    public function getNextRunDate($currentTime = 'now', int $nth = 0, bool $allowCurrentDate = false, $timeZone = null): DateTime
    {}

    /**
     * Get a previous run date relative to the current date or a specific date.
     *
     * @param string|\DateTimeInterface $currentTime      Relative calculation date
     * @param int                       $nth              Number of matches to skip before returning
     * @param bool                      $allowCurrentDate Set to TRUE to return the
     *                                                    current date if it matches the cron expression
     * @param null|string               $timeZone         TimeZone to use instead of the system default
     *
     * @throws \RuntimeException on too many iterations
     * @throws \Exception
     *
     * @return \DateTime
     *
     * @see \Cron\CronExpression::getNextRunDate
     */
    public function getPreviousRunDate($currentTime = 'now', int $nth = 0, bool $allowCurrentDate = false, $timeZone = null): DateTime
    {}

    /**
     * Get multiple run dates starting at the current date or a specific date.
     *
     * @param int $total Set the total number of dates to calculate
     * @param string|\DateTimeInterface|null $currentTime Relative calculation date
     * @param bool $invert Set to TRUE to retrieve previous dates
     * @param bool $allowCurrentDate Set to TRUE to return the
     *                               current date if it matches the cron expression
     * @param null|string $timeZone TimeZone to use instead of the system default
     *
     * @return \DateTime[] Returns an array of run dates
     */
    public function getMultipleRunDates(int $total, $currentTime = 'now', bool $invert = false, bool $allowCurrentDate = false, $timeZone = null): array
    {}

    /**
     * Get all or part of the CRON expression.
     *
     * @param int|string|null $part specify the part to retrieve or NULL to get the full
     *                     cron schedule string
     *
     * @return null|string Returns the CRON expression, a part of the
     *                     CRON expression, or NULL if the part was specified but not found
     */
    public function getExpression($part = null): ?string
    {}

    /**
     * Gets the parts of the cron expression as an array.
     *
     * @return string[]
     *   The array of parts that make up this expression.
     */
    public function getParts()
    {}

    /**
     * Helper method to output the full expression.
     *
     * @return string Full CRON expression
     */
    public function __toString(): string
    {}

    /**
     * Determine if the cron is due to run based on the current date or a
     * specific date.  This method assumes that the current number of
     * seconds are irrelevant, and should be called once per minute.
     *
     * @param string|\DateTimeInterface $currentTime Relative calculation date
     * @param null|string               $timeZone    TimeZone to use instead of the system default
     *
     * @return bool Returns TRUE if the cron is due to run or FALSE if not
     */
    public function isDue($currentTime = 'now', $timeZone = null): bool
    {}

    /**
     * Get the next or previous run date of the expression relative to a date.
     *
     * @param string|\DateTimeInterface|null $currentTime Relative calculation date
     * @param int $nth Number of matches to skip before returning
     * @param bool $invert Set to TRUE to go backwards in time
     * @param bool $allowCurrentDate Set to TRUE to return the
     *                               current date if it matches the cron expression
     * @param string|null $timeZone  TimeZone to use instead of the system default
     *
     * @throws \RuntimeException on too many iterations
     * @throws Exception
     *
     * @return \DateTime
     */
    protected function getRunDate($currentTime = null, int $nth = 0, bool $invert = false, bool $allowCurrentDate = false, $timeZone = null): DateTime
    {}

    /**
     * Workout what timeZone should be used.
     *
     * @param string|\DateTimeInterface|null $currentTime Relative calculation date
     * @param string|null $timeZone TimeZone to use instead of the system default
     *
     * @return string
     */
    protected function determineTimeZone($currentTime, ?string $timeZone): string
    {}
}
