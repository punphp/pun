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
 * An immmutable DateTime with stricter error handling and return types than the native one.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
final class DatePoint extends \DateTimeImmutable
{
    /**
     * @throws \DateMalformedStringException When $datetime is invalid
     */
    public function __construct(string $datetime = 'now', ?\DateTimeZone $timezone = null, ?parent $reference = null)
    {}

    /**
     * @throws \DateMalformedStringException When $format or $datetime are invalid
     */
    public static function createFromFormat(string $format, string $datetime, ?\DateTimeZone $timezone = null): static
    {}

    public static function createFromInterface(\DateTimeInterface $object): static
    {}

    public static function createFromMutable(\DateTime $object): static
    {}

    public static function createFromTimestamp(int|float $timestamp): static
    {}

    public function add(\DateInterval $interval): static
    {}

    public function sub(\DateInterval $interval): static
    {}

    /**
     * @throws \DateMalformedStringException When $modifier is invalid
     */
    public function modify(string $modifier): static
    {}

    public function setTimestamp(int $value): static
    {}

    public function setDate(int $year, int $month, int $day): static
    {}

    public function setISODate(int $year, int $week, int $day = 1): static
    {}

    public function setTime(int $hour, int $minute, int $second = 0, int $microsecond = 0): static
    {}

    public function setTimezone(\DateTimeZone $timezone): static
    {}

    public function getTimezone(): \DateTimeZone
    {}

    public function setMicrosecond(int $microsecond): static
    {}
}
