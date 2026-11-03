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

use Carbon\CarbonInterface;
use Carbon\CarbonTimeZone;
use Carbon\Factory;
use Carbon\FactoryImmutable;
use Closure;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;

trait Test
{
    ///////////////////////////////////////////////////////////////////
    ///////////////////////// TESTING AIDS ////////////////////////////
    ///////////////////////////////////////////////////////////////////

    /**
     * Set a Carbon instance (real or mock) to be returned when a "now"
     * instance is created. The provided instance will be returned
     * specifically under the following conditions:
     *   - A call to the static now() method, ex. Carbon::now()
     *   - When a null (or blank string) is passed to the constructor or parse(), ex. new Carbon(null)
     *   - When the string "now" is passed to the constructor or parse(), ex. new Carbon('now')
     *   - When a string containing the desired time is passed to Carbon::parse().
     *
     * Only the moment is mocked with setTestNow(), the timezone will still be the one passed
     * as parameter of date_default_timezone_get() as a fallback (see setTestNowAndTimezone()).
     *
     * To clear the test instance call this method using the default
     * parameter of null.
     *
     * /!\ Use this method for unit tests only.
     *
     * @param DateTimeInterface|Closure|static|string|false|null $testNow real or mock Carbon instance
     */
    public static function setTestNow(mixed $testNow = null): void
    {}

    /**
     * Set a Carbon instance (real or mock) to be returned when a "now"
     * instance is created. The provided instance will be returned
     * specifically under the following conditions:
     *   - A call to the static now() method, ex. Carbon::now()
     *   - When a null (or blank string) is passed to the constructor or parse(), ex. new Carbon(null)
     *   - When the string "now" is passed to the constructor or parse(), ex. new Carbon('now')
     *   - When a string containing the desired time is passed to Carbon::parse().
     *
     * It will also align default timezone (e.g. call date_default_timezone_set()) with
     * the second argument or if null, with the timezone of the given date object.
     *
     * To clear the test instance call this method using the default
     * parameter of null.
     *
     * /!\ Use this method for unit tests only.
     *
     * @param DateTimeInterface|Closure|static|string|false|null $testNow real or mock Carbon instance
     */
    public static function setTestNowAndTimezone($testNow = null, $timezone = null): void
    {}

    /**
     * Temporarily sets a static date to be used within the callback.
     * Using setTestNow to set the date, executing the callback, then
     * clearing the test instance.
     *
     * /!\ Use this method for unit tests only.
     *
     * @template T
     *
     * @param DateTimeInterface|Closure|static|string|false|null $testNow  real or mock Carbon instance
     * @param Closure(): T                                       $callback
     *
     * @return T
     */
    public static function withTestNow(mixed $testNow, callable $callback): mixed
    {}

    /**
     * Get the Carbon instance (real or mock) to be returned when a "now"
     * instance is created.
     *
     * @return Closure|CarbonInterface|null the current instance used for testing
     */
    public static function getTestNow(): Closure|CarbonInterface|null
    {}

    /**
     * Determine if there is a valid test instance set. A valid test instance
     * is anything that is not null.
     *
     * @return bool true if there is a test instance, otherwise false
     */
    public static function hasTestNow(): bool
    {}

    /**
     * Get the mocked date passed in setTestNow() and if it's a Closure, execute it.
     */
    protected static function getMockedTestNow(DateTimeZone|string|int|null $timezone): ?CarbonInterface
    {}

    private function mockConstructorParameters(&$time, ?CarbonTimeZone $timezone): void
    {}

    private static function getMockedTestNowClone($timezone): CarbonInterface|self|null
    {}

    private function nowFromClock(?CarbonTimeZone $timezone): ?DateTimeImmutable
    {}
}
