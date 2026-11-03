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

enum Unit: string
{
    case Microsecond = 'microsecond';
    case Millisecond = 'millisecond';
    case Second = 'second';
    case Minute = 'minute';
    case Hour = 'hour';
    case Day = 'day';
    case Week = 'week';
    case Month = 'month';
    case Quarter = 'quarter';
    case Year = 'year';
    case Decade = 'decade';
    case Century = 'century';
    case Millennium = 'millennium';

    public static function toName(self|string $unit): string
    {}

    /** @internal */
    public static function toNameIfUnit(mixed $unit): mixed
    {}

    public static function fromName(string $name, ?string $locale = null): self
    {}

    public function singular(?string $locale = null): string
    {}

    public function plural(?string $locale = null): string
    {}

    public function interval(int|float $value = 1): CarbonInterval
    {}

    public function locale(string $locale): CarbonInterval
    {}

    public function toPeriod(...$params): CarbonPeriod
    {}

    public function stepBy(mixed $interval, Unit|string|null $unit = null): CarbonPeriod
    {}
}
