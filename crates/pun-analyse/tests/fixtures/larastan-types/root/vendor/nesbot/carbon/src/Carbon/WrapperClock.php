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

use DateTime;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Psr\Clock\ClockInterface as PsrClockInterface;
use RuntimeException;
use Symfony\Component\Clock\ClockInterface;

final class WrapperClock implements ClockInterface
{
    public function __construct(
        private PsrClockInterface|Factory|DateTimeInterface $currentClock,
    ) {}

    public function unwrap(): PsrClockInterface|Factory|DateTimeInterface
    {}

    public function getFactory(): Factory
    {}

    private function nowRaw(): DateTimeInterface
    {}

    public function now(): DateTimeImmutable
    {}

    /**
     * @template T of CarbonInterface
     *
     * @param class-string<T> $class
     *
     * @return T
     */
    public function nowAs(string $class, DateTimeZone|string|int|null $timezone = null): CarbonInterface
    {}

    public function nowAsCarbon(DateTimeZone|string|int|null $timezone = null): CarbonInterface
    {}

    private function dateAsCarbon(DateTimeInterface $date, DateTimeZone|string|int|null $timezone): CarbonInterface
    {}

    public function sleep(float|int $seconds): void
    {}

    public function withTimeZone(DateTimeZone|string $timezone): static
    {}

    private function addSeconds(DateTimeInterface $date, float|int $seconds): DateTimeInterface
    {}
}
