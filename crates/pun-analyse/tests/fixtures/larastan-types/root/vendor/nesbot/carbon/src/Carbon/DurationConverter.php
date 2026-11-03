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
use Carbon\Exceptions\InvalidIntervalException;
use DateInterval;
use Time\Duration;

/** @internal */
final class DurationConverter
{
    public static function getDurationUnits(Duration $duration): array
    {}

    public static function getDurationIsoSpec(Duration $duration): string
    {}

    public static function createDurationFromInterval(DateInterval|Duration $interval): Duration
    {}

    public static function createDurationFromSeconds(int $seconds, int $nanoseconds = 0, bool $inverted = false): Duration
    {}
}
