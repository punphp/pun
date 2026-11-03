<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TimeLimit;

use const SIGALRM;
use function ceil;
use function function_exists;
use function getmypid;
use function hrtime;
use function pcntl_alarm;
use function pcntl_async_signals;
use function pcntl_signal;
use function sprintf;
use PHPUnit\Event\Facade;

/**
 * Enforces the wall-clock time limit for the entire test run.
 *
 * The time limit is checked whenever a test has finished. When the pcntl
 * extension is available, a SIGALRM-based hard stop is additionally armed
 * for each test that runs in the same process as the test runner, so that
 * a test that hangs cannot keep the test run from ending.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TimeLimitHandler
{
    private static ?self $instance = null;
    private readonly Facade $facade;

    /**
     * @var positive-int
     */
    private readonly int $timeLimit;
    private readonly int $deadline;
    private readonly false|int $pid;
    private readonly bool $canUseAlarm;
    private bool $alarmFired = false;
    private bool $exceeded   = false;

    /**
     * @param positive-int $timeLimit
     */
    public static function init(Facade $facade, int $timeLimit): void
    {}

    /**
     * Arms the hard stop for a test that is about to run in the same process
     * as the test runner. This is a no-op when no time limit is configured or
     * when the pcntl extension is not available.
     */
    public static function armAlarm(): void
    {}

    /**
     * Disarms the hard stop after a test that ran in the same process as the
     * test runner has finished.
     */
    public static function disarmAlarm(): void
    {}

    /**
     * @param positive-int $timeLimit
     */
    private function __construct(Facade $facade, int $timeLimit)
    {}

    public function testFinished(): void
    {}

    private function arm(): void
    {}

    private function disarm(): void
    {}

    /**
     * @return positive-int
     */
    private function secondsUntilDeadline(): int
    {}
}
