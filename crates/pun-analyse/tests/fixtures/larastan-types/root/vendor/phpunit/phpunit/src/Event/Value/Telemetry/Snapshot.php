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
final readonly class Snapshot
{
    private HRTime $time;
    private MemoryUsage $memoryUsage;
    private MemoryUsage $peakMemoryUsage;
    private GarbageCollectorStatus $garbageCollectorStatus;
    private CpuTime $userCpuTime;
    private CpuTime $systemCpuTime;
    private CpuTime $totalCpuTime;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(HRTime $time, MemoryUsage $memoryUsage, MemoryUsage $peakMemoryUsage, GarbageCollectorStatus $garbageCollectorStatus, CpuTime $userCpuTime, CpuTime $systemCpuTime, CpuTime $totalCpuTime)
    {}

    public function time(): HRTime
    {}

    public function memoryUsage(): MemoryUsage
    {}

    public function peakMemoryUsage(): MemoryUsage
    {}

    public function garbageCollectorStatus(): GarbageCollectorStatus
    {}

    public function userCpuTime(): CpuTime
    {}

    public function systemCpuTime(): CpuTime
    {}

    public function totalCpuTime(): CpuTime
    {}
}
