<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output\Compact\ProgressPrinter;

use const PHP_EOL;
use function assert;
use function rtrim;
use function str_starts_with;
use function strlen;
use function substr;
use function trim;
use PHPUnit\Event\Code\Test;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Facade;
use PHPUnit\Event\Test\AfterLastTestMethodErrored;
use PHPUnit\Event\Test\AfterLastTestMethodFailed;
use PHPUnit\Event\Test\BeforeFirstTestMethodErrored;
use PHPUnit\Event\Test\BeforeFirstTestMethodFailed;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\PreparationStarted;
use PHPUnit\Event\Test\PrintedUnexpectedOutput;
use PHPUnit\TextUI\Output\Compact\Renderer;
use PHPUnit\TextUI\Output\Printer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ProgressPrinter
{
    private readonly Printer $printer;
    private readonly Renderer $renderer;
    private readonly bool $displayUnexpectedOutput;
    private ?Test $currentTest      = null;
    private bool $hasPrintedRecords = false;

    public function __construct(Printer $printer, Facade $facade, bool $displayUnexpectedOutput)
    {}

    public function testPreparationStarted(PreparationStarted $event): void
    {}

    public function testErrored(Errored $event): void
    {}

    public function testFailed(Failed $event): void
    {}

    public function testPrintedUnexpectedOutput(PrintedUnexpectedOutput $event): void
    {}

    public function beforeFirstTestMethodErrored(BeforeFirstTestMethodErrored $event): void
    {}

    public function beforeFirstTestMethodFailed(BeforeFirstTestMethodFailed $event): void
    {}

    public function afterLastTestMethodErrored(AfterLastTestMethodErrored $event): void
    {}

    public function afterLastTestMethodFailed(AfterLastTestMethodFailed $event): void
    {}

    /**
     * Separates the records printed during the run from the summary that follows.
     */
    public function testRunnerExecutionFinished(): void
    {}

    private function printError(string $title, Throwable $throwable): void
    {}

    private function printFailure(string $title, Throwable $throwable): void
    {}

    private function registerSubscribers(Facade $facade): void
    {}
}
