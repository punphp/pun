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

use function getrusage;
use function is_int;
use function sprintf;
use PHPUnit\Event\RuntimeException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class SystemCpuTimeMeter implements CpuTimeMeter
{
    /**
     * @throws RuntimeException
     */
    public function userCpuTime(): CpuTime
    {}

    /**
     * @throws RuntimeException
     */
    public function systemCpuTime(): CpuTime
    {}

    /**
     * @param 'ru_stime.tv_sec'|'ru_utime.tv_sec'   $secondsKey
     * @param 'ru_stime.tv_usec'|'ru_utime.tv_usec' $microsecondsKey
     *
     * @throws RuntimeException
     */
    private function cpuTime(string $secondsKey, string $microsecondsKey): CpuTime
    {}
}
