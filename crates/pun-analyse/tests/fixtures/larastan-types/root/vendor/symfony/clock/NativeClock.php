<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Clock;

/**
 * A clock that relies the system time.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
final class NativeClock implements ClockInterface
{
    private \DateTimeZone $timezone;

    /**
     * @throws \DateInvalidTimeZoneException When $timezone is invalid
     */
    public function __construct(\DateTimeZone|string|null $timezone = null)
    {}

    public function now(): DatePoint
    {}

    public function sleep(float|int $seconds): void
    {}

    /**
     * @throws \DateInvalidTimeZoneException When $timezone is invalid
     */
    public function withTimeZone(\DateTimeZone|string $timezone): static
    {}
}
