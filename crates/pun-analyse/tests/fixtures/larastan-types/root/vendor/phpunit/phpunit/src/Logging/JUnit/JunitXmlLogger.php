<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging\JUnit;

use const PHP_EOL;
use function assert;
use function basename;
use function sprintf;
use function str_replace;
use function trim;
use DOMDocument;
use DOMElement;
use PHPUnit\Event\Code\Test;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Facade;
use PHPUnit\Event\InvalidArgumentException;
use PHPUnit\Event\Telemetry\HRTime;
use PHPUnit\Event\Telemetry\Info;
use PHPUnit\Event\Test\AttemptErrored;
use PHPUnit\Event\Test\AttemptFailed;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\Finished;
use PHPUnit\Event\Test\MarkedIncomplete;
use PHPUnit\Event\Test\PreparationStarted;
use PHPUnit\Event\Test\Prepared;
use PHPUnit\Event\Test\PrintedUnexpectedOutput;
use PHPUnit\Event\Test\Skipped;
use PHPUnit\Event\TestSuite\Skipped as TestSuiteSkipped;
use PHPUnit\Event\TestSuite\Started;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Xml;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class JunitXmlLogger
{
    private readonly Printer $printer;
    private DOMDocument $document;
    private DOMElement $root;

    /**
     * @var array<int, DOMElement>
     */
    private array $testSuites = [];

    /**
     * @var array<int, int>
     */
    private array $testSuiteTests = [0];

    /**
     * @var array<int, int>
     */
    private array $testSuiteAssertions = [0];

    /**
     * @var array<int, int>
     */
    private array $testSuiteErrors = [0];

    /**
     * @var array<int, int>
     */
    private array $testSuiteFailures = [0];

    /**
     * @var array<int, int>
     */
    private array $testSuiteSkipped = [0];

    /**
     * @var array<int, float>
     */
    private array $testSuiteTimes        = [0.0];
    private int $testSuiteLevel          = 0;
    private ?DOMElement $currentTestCase = null;
    private ?HRTime $time                = null;
    private bool $prepared               = false;
    private bool $preparationFailed      = false;
    private ?string $unexpectedOutput    = null;

    /**
     * Wall-clock time of the failed attempts that preceded the final attempt
     * of a retried test. This is added to the time of the final attempt so
     * that the reported duration covers all attempts.
     */
    private float $retriedAttemptsTime = 0.0;

    public function __construct(Printer $printer, Facade $facade)
    {}

    public function flush(): void
    {}

    public function testSuiteStarted(Started $event): void
    {}

    public function testSuiteSkipped(TestSuiteSkipped $event): void
    {}

    public function testSuiteFinished(): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testPreparationStarted(PreparationStarted $event): void
    {}

    public function testPreparationErrored(): void
    {}

    public function testPreparationFailed(): void
    {}

    public function testPrepared(): void
    {}

    public function testAttemptFailed(AttemptFailed $event): void
    {}

    public function testAttemptErrored(AttemptErrored $event): void
    {}

    public function testPrintedUnexpectedOutput(PrintedUnexpectedOutput $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testFinished(Finished $event): void
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
    private function handleFinish(Info $telemetryInfo, int $numberOfAssertionsPerformed): void
    {}

    private function registerSubscribers(Facade $facade): void
    {}

    private function createDocument(): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function handleFault(Errored|Failed $event, string $type): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function handleIncompleteOrSkipped(MarkedIncomplete|Skipped $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function testAsString(Test $test): string
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function name(Test $test): string
    {}

    /**
     * @throws InvalidArgumentException
     *
     * @phpstan-assert !null $this->currentTestCase
     */
    private function createTestCase(Errored|Failed|MarkedIncomplete|PreparationStarted|Prepared|Skipped $event): void
    {}
}
