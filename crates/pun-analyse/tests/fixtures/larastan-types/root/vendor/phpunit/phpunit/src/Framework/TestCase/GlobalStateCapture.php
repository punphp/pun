<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestCase;

use const PHP_EOL;
use function array_keys;
use function array_reverse;
use function count;
use function defined;
use function is_string;
use function restore_exception_handler;
use function set_exception_handler;
use function str_starts_with;
use function trim;
use PHPUnit\Event;
use PHPUnit\Framework\TestCase;
use PHPUnit\Runner\ErrorHandler;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use PHPUnit\Util\DifferBuilder;
use PHPUnit\Util\Exporter;
use SebastianBergmann\Comparator\Factory as ComparatorFactory;
use SebastianBergmann\GlobalState\ExcludeList as GlobalStateExcludeList;
use SebastianBergmann\GlobalState\Restorer;
use SebastianBergmann\GlobalState\Snapshot;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class GlobalStateCapture
{
    private ?bool $backupGlobals = null;

    /**
     * @var list<string>
     */
    private array $backupGlobalsExcludeList = [];
    private ?bool $backupStaticProperties   = null;

    /**
     * @var array<class-string, list<non-empty-string>>
     */
    private array $backupStaticPropertiesExcludeList = [];
    private ?Snapshot $snapshot                      = null;

    /**
     * @var null|list<callable>
     */
    private ?array $backupGlobalExceptionHandlers = null;

    public function setBackupGlobals(bool $backupGlobals): void
    {}

    /**
     * @param list<string> $backupGlobalsExcludeList
     */
    public function setBackupGlobalsExcludeList(array $backupGlobalsExcludeList): void
    {}

    public function setBackupStaticProperties(bool $backupStaticProperties): void
    {}

    /**
     * @param array<class-string, list<non-empty-string>> $backupStaticPropertiesExcludeList
     */
    public function setBackupStaticPropertiesExcludeList(array $backupStaticPropertiesExcludeList): void
    {}

    /**
     * @throws Throwable
     */
    public function snapshotGlobals(TestCase $test, Event\Emitter $emitter, bool $inIsolation, ?bool $runTestInSeparateProcess): void
    {}

    /**
     * @throws Throwable
     */
    public function restoreGlobals(TestCase $test, Event\Emitter $emitter): void
    {}

    public function snapshotErrorHandlers(TestCase $test, Event\Emitter $emitter): void
    {}

    public function restoreErrorHandlers(TestCase $test, Event\Emitter $emitter, bool $inIsolation): void
    {}

    /**
     * @throws Throwable
     */
    public function createSnapshot(TestCase $test, Event\Emitter $emitter, bool $backupGlobals): Snapshot
    {}

    private function compareSnapshots(TestCase $test, Event\Emitter $emitter, Snapshot $before, Snapshot $after): void
    {}

    /**
     * @param array<mixed>     $before
     * @param array<mixed>     $after
     * @param non-empty-string $header
     */
    private static function compareSnapshotPart(TestCase $test, Event\Emitter $emitter, array $before, array $after, string $header): void
    {}

    /**
     * @return list<callable>
     */
    private static function activeExceptionHandlers(): array
    {}
}
