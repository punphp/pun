<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging\TeamCity;

use function assert;
use function getmypid;
use function ini_get;
use function is_a;
use function round;
use function sprintf;
use function str_replace;
use function stripos;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Event;
use PHPUnit\Event\Facade;
use PHPUnit\Event\InvalidArgumentException;
use PHPUnit\Event\Telemetry\HRTime;
use PHPUnit\Event\Test\BeforeFirstTestMethodErrored;
use PHPUnit\Event\Test\BeforeFirstTestMethodFailed;
use PHPUnit\Event\Test\ConsideredRisky;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\Finished;
use PHPUnit\Event\Test\MarkedIncomplete;
use PHPUnit\Event\Test\PreparationStarted;
use PHPUnit\Event\Test\Skipped;
use PHPUnit\Event\TestSuite\Finished as TestSuiteFinished;
use PHPUnit\Event\TestSuite\Skipped as TestSuiteSkipped;
use PHPUnit\Event\TestSuite\Started as TestSuiteStarted;
use PHPUnit\Event\TestSuite\TestSuiteForRepeatedTestMethod;
use PHPUnit\Event\TestSuite\TestSuiteForRetriedTestMethod;
use PHPUnit\Event\TestSuite\TestSuiteForTestClass;
use PHPUnit\Event\TestSuite\TestSuiteForTestMethodWithDataProvider;
use PHPUnit\Framework\Exception as FrameworkException;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Sanitizer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TeamCityLogger
{
    private readonly Printer $printer;
    private bool $isSummaryTestCountPrinted = false;
    private ?HRTime $time                   = null;
    private ?int $flowId                    = null;
    private bool $testStartedEmitted        = false;
    private bool $prepared                  = false;
    private bool $preparationFailed         = false;

    public function __construct(Printer $printer, Facade $facade)
    {}

    public function testSuiteStarted(TestSuiteStarted $event): void
    {}

    public function testSuiteFinished(TestSuiteFinished $event): void
    {}

    public function testPreparationStarted(PreparationStarted $event): void
    {}

    public function testPreparationErrored(): void
    {}

    public function testPreparationFailed(): void
    {}

    public function testPrepared(): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testMarkedIncomplete(MarkedIncomplete $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testSkipped(Skipped $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testSuiteSkipped(TestSuiteSkipped $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function beforeFirstTestMethodErrored(BeforeFirstTestMethodErrored $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function beforeFirstTestMethodFailed(BeforeFirstTestMethodFailed $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testErrored(Errored $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testFailed(Failed $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testConsideredRisky(ConsideredRisky $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testFinished(Finished $event): void
    {}

    public function flush(): void
    {}

    private function registerSubscribers(Facade $facade): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function writeBeforeFirstTestMethodHookFailure(Event $event, string $name, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function writeTestFinishedIfPreparationDidNotComplete(Errored|Failed|MarkedIncomplete|Skipped $event): void
    {}

    private function setFlowId(): void
    {}

    /**
     * @param array<non-empty-string, int|string> $parameters
     */
    private function writeMessage(string $eventName, array $parameters = []): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function duration(Event $event): int
    {}

    private function escape(string $string): string
    {}

    private function message(Throwable $throwable): string
    {}

    private function details(Throwable $throwable): string
    {}
}
