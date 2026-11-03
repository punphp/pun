<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Telemetry;

use function sprintf;
use PHPUnit\Event\InvalidArgumentException;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class CpuTime
{
    /** @var non-negative-int */
    private int $seconds;

    /** @var non-negative-int */
    private int $nanoseconds;

    /**
     * @throws InvalidArgumentException
     */
    public static function fromSecondsAndNanoseconds(int $seconds, int $nanoseconds): self
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function __construct(int $seconds, int $nanoseconds)
    {}

    public function seconds(): int
    {}

    public function nanoseconds(): int
    {}

    public function asFloat(): float
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function add(self $other): self
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function diff(self $other): self
    {}

    /**
     * @phpstan-assert non-negative-int $value
     *
     * @throws InvalidArgumentException
     */
    private function ensureNotNegative(int $value, string $type): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function ensureNanoSecondsInRange(int $nanoseconds): void
    {}
}
