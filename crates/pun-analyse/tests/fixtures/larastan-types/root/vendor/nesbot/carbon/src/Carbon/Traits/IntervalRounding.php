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

use Carbon\CarbonInterval;
use Carbon\Exceptions\InvalidIntervalException;
use DateInterval;
use Time\Duration;

/**
 * Trait to call rounding methods to interval or the interval of a period.
 */
trait IntervalRounding
{
    protected function callRoundMethod(string $method, array $parameters): ?static
    {}

    protected function roundWith(DateInterval|Duration|string|float|int $precision, callable|string $function): ?static
    {}
}
