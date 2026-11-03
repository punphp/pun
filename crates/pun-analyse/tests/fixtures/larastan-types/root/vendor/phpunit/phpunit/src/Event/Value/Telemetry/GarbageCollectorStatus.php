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

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class GarbageCollectorStatus
{
    private int $runs;
    private int $collected;
    private int $threshold;
    private int $roots;
    private float $applicationTime;
    private float $collectorTime;
    private float $destructorTime;
    private float $freeTime;
    private bool $running;
    private bool $protected;
    private bool $full;
    private int $bufferSize;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(int $runs, int $collected, int $threshold, int $roots, float $applicationTime, float $collectorTime, float $destructorTime, float $freeTime, bool $running, bool $protected, bool $full, int $bufferSize)
    {}

    public function runs(): int
    {}

    public function collected(): int
    {}

    public function threshold(): int
    {}

    public function roots(): int
    {}

    public function applicationTime(): float
    {}

    public function collectorTime(): float
    {}

    public function destructorTime(): float
    {}

    public function freeTime(): float
    {}

    public function isRunning(): bool
    {}

    public function isProtected(): bool
    {}

    public function isFull(): bool
    {}

    public function bufferSize(): int
    {}
}
