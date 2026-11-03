<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TestIndex;

use function is_file;
use function str_ends_with;
use Closure;
use PHPUnit\Event\EventCollection;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Event\Test\DataProviderMethodCalled;
use PHPUnit\Event\Test\DataProviderMethodFinished;
use PHPUnit\Runner\Exception;
use PHPUnit\Runner\TestSuiteLoader;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DefaultTestFileSkipper implements TestFileSkipper
{
    private readonly EventFacade $eventFacade;
    private readonly TestIndex $index;
    private readonly GroupPruner $groupPruner;
    private readonly NameFilterPruner $nameFilterPruner;

    /**
     * @var ?non-empty-string
     */
    private ?string $recording = null;

    public function __construct(EventFacade $eventFacade, TestIndex $index, GroupPruner $groupPruner, NameFilterPruner $nameFilterPruner)
    {}

    /**
     * A file is only skipped when it is indexed, the entry for it is still
     * valid, and none of the tests in it can be selected.
     *
     * Either way of selecting tests is enough on its own to establish that:
     * a test that is in no selected group is not run whether or not its name
     * matches, and the other way round.
     *
     * @param non-empty-string       $file
     * @param list<non-empty-string> $groupsFromConfiguration
     */
    public function canSkipLoading(string $file, array $groupsFromConfiguration): bool
    {}

    /**
     * @template T
     *
     * @param non-empty-string $file
     * @param Closure(): T     $load
     *
     * @throws Throwable
     *
     * @return T
     */
    public function record(string $file, Closure $load): mixed
    {}

    public function persist(): void
    {}

    /**
     * Files that are already indexed are not indexed again: the entry for a
     * file is only handed out while it is still valid, so an entry that exists
     * describes the file as it is now.
     *
     * A PHPT file is not a PHP file and is never loaded as one, so it is not
     * indexed either. The condition for that is the one that TestSuite uses to
     * decide whether a file is loaded as PHP.
     *
     * @param non-empty-string $file
     */
    private function startRecording(string $file): void
    {}

    private function stopRecording(): void
    {}

    /**
     * What was collected while the file was being loaded is still forwarded:
     * the file is not indexed, but what PHPUnit had to say about it before
     * loading it failed must not be swallowed. Leaving the event facade in the
     * state it was in before is what makes a failure that is reported instead
     * of ending the run harmless.
     */
    private function abortRecording(): void
    {}

    /**
     * Loading a test file emits an event for every data provider method that
     * was called. Any other event means PHPUnit had something to say about the
     * file, and a file PHPUnit has something to say about is never skipped: it
     * would otherwise depend on the state of the index whether that is said.
     *
     * Treating an unknown event as something to say keeps this true for events
     * that do not exist yet.
     *
     * What this keeps the same is what PHPUnit reports about a run and which
     * tests it runs, not the events the run emits. A skipped file runs none of
     * its data provider methods, so the events for them are not emitted either.
     * The number of tests a test suite has before it is filtered depends on the
     * index as well, and does so for every skipped file rather than only for
     * the ones that have a data provider. Both are visible to --debug and to
     * the event loggers.
     */
    private static function madePhpUnitWarn(EventCollection $events): bool
    {}
}
