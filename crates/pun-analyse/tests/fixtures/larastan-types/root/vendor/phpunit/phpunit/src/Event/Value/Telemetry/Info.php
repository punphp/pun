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

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Info
{
    private Snapshot $current;
    private Duration $durationSinceStart;
    private MemoryUsage $memorySinceStart;
    private Duration $durationSincePrevious;
    private MemoryUsage $memorySincePrevious;
    private CpuTime $userCpuTimeSinceStart;
    private CpuTime $systemCpuTimeSinceStart;
    private CpuTime $totalCpuTimeSinceStart;
    private CpuTime $userCpuTimeSincePrevious;
    private CpuTime $systemCpuTimeSincePrevious;
    private CpuTime $totalCpuTimeSincePrevious;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Snapshot $current, Duration $durationSinceStart, MemoryUsage $memorySinceStart, Duration $durationSincePrevious, MemoryUsage $memorySincePrevious, CpuTime $userCpuTimeSinceStart, CpuTime $systemCpuTimeSinceStart, CpuTime $totalCpuTimeSinceStart, CpuTime $userCpuTimeSincePrevious, CpuTime $systemCpuTimeSincePrevious, CpuTime $totalCpuTimeSincePrevious)
    {}

    public function time(): HRTime
    {}

    public function memoryUsage(): MemoryUsage
    {}

    public function peakMemoryUsage(): MemoryUsage
    {}

    public function durationSinceStart(): Duration
    {}

    public function memoryUsageSinceStart(): MemoryUsage
    {}

    public function durationSincePrevious(): Duration
    {}

    public function memoryUsageSincePrevious(): MemoryUsage
    {}

    public function garbageCollectorStatus(): GarbageCollectorStatus
    {}

    public function userCpuTime(): CpuTime
    {}

    public function systemCpuTime(): CpuTime
    {}

    public function totalCpuTime(): CpuTime
    {}

    public function userCpuTimeSinceStart(): CpuTime
    {}

    public function systemCpuTimeSinceStart(): CpuTime
    {}

    public function totalCpuTimeSinceStart(): CpuTime
    {}

    public function userCpuTimeSincePrevious(): CpuTime
    {}

    public function systemCpuTimeSincePrevious(): CpuTime
    {}

    public function totalCpuTimeSincePrevious(): CpuTime
    {}

    public function asString(): string
    {}
}
