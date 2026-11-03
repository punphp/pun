<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI;

use const PHP_EOL;
use const PHP_VERSION;
use const SIGINT;
use function array_reverse;
use function class_exists;
use function count;
use function defined;
use function dirname;
use function explode;
use function function_exists;
use function getmypid;
use function is_array;
use function is_file;
use function is_string;
use function method_exists;
use function pcntl_async_signals;
use function pcntl_signal;
use function printf;
use function realpath;
use function sprintf;
use function str_contains;
use function str_starts_with;
use function trim;
use function unlink;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\EventFacadeIsSealedException;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Event\UnknownSubscriberTypeException;
use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\TestSuite;
use PHPUnit\Logging\EventLogger;
use PHPUnit\Logging\JUnit\JunitXmlLogger;
use PHPUnit\Logging\OpenTestReporting\CannotOpenUriForWritingException;
use PHPUnit\Logging\OpenTestReporting\OtrXmlLogger;
use PHPUnit\Logging\TeamCity\TeamCityLogger;
use PHPUnit\Logging\TestDox\HtmlRenderer as TestDoxHtmlRenderer;
use PHPUnit\Logging\TestDox\PlainTextRenderer as TestDoxTextRenderer;
use PHPUnit\Logging\TestDox\TestResultCollector as TestDoxResultCollector;
use PHPUnit\Metadata\Api\Groups;
use PHPUnit\Runner\Baseline\CannotLoadBaselineException;
use PHPUnit\Runner\Baseline\Generator as BaselineGenerator;
use PHPUnit\Runner\Baseline\Reader;
use PHPUnit\Runner\Baseline\Writer;
use PHPUnit\Runner\CodeCoverage;
use PHPUnit\Runner\CodeCoverageInitializationStatus;
use PHPUnit\Runner\DeprecationCollector\Facade as DeprecationCollector;
use PHPUnit\Runner\DeprecationFilter;
use PHPUnit\Runner\DirectoryDoesNotExistException;
use PHPUnit\Runner\ErrorHandler;
use PHPUnit\Runner\Extension\ExtensionBootstrapper;
use PHPUnit\Runner\Extension\ExtensionCapabilities;
use PHPUnit\Runner\Extension\ExtensionFacade;
use PHPUnit\Runner\Extension\PharLoader;
use PHPUnit\Runner\GarbageCollection\GarbageCollectionHandler;
use PHPUnit\Runner\IssueTriggerResolver\Resolver;
use PHPUnit\Runner\PhpConfiguration\PhpConfigurationChecker;
use PHPUnit\Runner\Phpt\TestCase as PhptTestCase;
use PHPUnit\Runner\TestIndex\DefaultTestFileSkipper;
use PHPUnit\Runner\TestIndex\GroupPruner;
use PHPUnit\Runner\TestIndex\NameFilterPruner;
use PHPUnit\Runner\TestIndex\NullTestFileSkipper;
use PHPUnit\Runner\TestIndex\TestFileSkipper;
use PHPUnit\Runner\TestIndex\TestIndex;
use PHPUnit\Runner\TestRunHistory\DefaultTestRunHistory;
use PHPUnit\Runner\TestRunHistory\NullTestRunHistory;
use PHPUnit\Runner\TestRunHistory\TestRunHistory;
use PHPUnit\Runner\TestRunHistory\TestRunHistoryHandler;
use PHPUnit\Runner\TestSuiteSorter;
use PHPUnit\Runner\TimeLimit\TimeLimitHandler;
use PHPUnit\Runner\Version;
use PHPUnit\TestRunner\IssueFilter;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;
use PHPUnit\TextUI\CliArguments\Builder;
use PHPUnit\TextUI\CliArguments\Configuration as CliConfiguration;
use PHPUnit\TextUI\CliArguments\Exception as ArgumentsException;
use PHPUnit\TextUI\CliArguments\XmlConfigurationFileFinder;
use PHPUnit\TextUI\Command\AtLeastVersionCommand;
use PHPUnit\TextUI\Command\CheckPhpConfigurationCommand;
use PHPUnit\TextUI\Command\GenerateConfigurationCommand;
use PHPUnit\TextUI\Command\ListGroupsCommand;
use PHPUnit\TextUI\Command\ListTestFilesCommand;
use PHPUnit\TextUI\Command\ListTestIdsCommand;
use PHPUnit\TextUI\Command\ListTestsAsTextCommand;
use PHPUnit\TextUI\Command\ListTestsAsXmlCommand;
use PHPUnit\TextUI\Command\ListTestSuitesCommand;
use PHPUnit\TextUI\Command\MigrateConfigurationCommand;
use PHPUnit\TextUI\Command\Result;
use PHPUnit\TextUI\Command\ShowHelpCommand;
use PHPUnit\TextUI\Command\ShowVersionCommand;
use PHPUnit\TextUI\Command\ValidateConfigurationCommand;
use PHPUnit\TextUI\Command\VersionCheckCommand;
use PHPUnit\TextUI\Command\WarmCodeCoverageCacheCommand;
use PHPUnit\TextUI\Configuration\BootstrapLoader;
use PHPUnit\TextUI\Configuration\BootstrapScriptDoesNotExistException;
use PHPUnit\TextUI\Configuration\BootstrapScriptException;
use PHPUnit\TextUI\Configuration\CodeCoverageFilterRegistry;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\Configuration\PhpHandler;
use PHPUnit\TextUI\Configuration\Registry;
use PHPUnit\TextUI\Configuration\TestSuiteBuilder;
use PHPUnit\TextUI\Output\DefaultPrinter;
use PHPUnit\TextUI\Output\Facade as OutputFacade;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\TextUI\XmlConfiguration\Configuration as XmlConfiguration;
use PHPUnit\TextUI\XmlConfiguration\DefaultConfiguration;
use PHPUnit\TextUI\XmlConfiguration\Loader;
use PHPUnit\Util\DifferBuilder;
use PHPUnit\Util\Http\PhpDownloader;
use SebastianBergmann\Timer\Timer;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Application
{
    private Emitter $emitter;

    public function __construct()
    {}

    /**
     * @param list<string> $argv
     */
    public function run(array $argv): int
    {}

    private function execute(Command\Command $command, bool $requiresResultCollectedFromEvents = false): never
    {}

    /**
     * @param list<string> $argv
     */
    private function buildCliConfiguration(array $argv): CliConfiguration
    {}

    private function loadXmlConfiguration(false|string $configurationFile): XmlConfiguration
    {}

    private function buildTestSuite(Configuration $configuration, CliConfiguration $cliConfiguration): TestSuite
    {}

    private function bootstrapExtensions(Configuration $configuration): ExtensionCapabilities
    {}

    private function executeCommandsThatOnlyRequireCliConfiguration(CliConfiguration $cliConfiguration, false|string $configurationFile): void
    {}

    private function executeCommandsThatDoNotRequireTheTestSuite(Configuration $configuration, CliConfiguration $cliConfiguration): void
    {}

    private function executeCommandsThatRequireTheTestSuite(Configuration $configuration, CliConfiguration $cliConfiguration, TestSuite $testSuite): void
    {}

    private function writeRuntimeInformation(Printer $printer, Configuration $configuration): void
    {}

    /**
     * @param ?list<string> $pharExtensions
     */
    private function writePharExtensionInformation(Printer $printer, ?array $pharExtensions): void
    {}

    private function writeMessage(Printer $printer, string $type, string $message): void
    {}

    private function writeRandomSeedInformation(Printer $printer, Configuration $configuration): void
    {}

    private function registerLogfileWriters(Configuration $configuration): void
    {}

    private function testDoxResultCollector(Configuration $configuration): ?TestDoxResultCollector
    {}

    /**
     * The index is only usable when there is somewhere to keep it, and it can
     * only save work when tests are selected by group: it answers whether a
     * test file can contribute a test to the run, which is a question only a
     * selection by group can answer without loading the file.
     */
    private function initializeTestIndex(Configuration $configuration, CliConfiguration $cliConfiguration): TestFileSkipper
    {}

    /**
     * The groups a test can be in for its test file to be worth loading.
     *
     * TestSuiteFilterProcessor selects by these same groups, but it adds a
     * filter of its own for --group, for --covers, for --uses, and for
     * --requires-php-extension: a test has to be selected by every one of the
     * options that were used. The pruner has them all in one list and asks only
     * whether a test is selected by any of them, so it keeps files that the
     * filters go on to take every test from.
     *
     * A value of --group that names several groups is one entry of that list
     * and keeps requiring all of them, which is what the filter for --group
     * requires as well. The list is therefore no less precise for that option
     * than the filter is, and still less precise than the filters are taken
     * together.
     *
     * That is the direction in which the index has to be wrong: leaving work
     * for the filters costs no more than the time it takes, while pruning a
     * file that has a test the filters would select would change which tests
     * are run.
     *
     * @return list<non-empty-string>
     */
    private function includedGroups(Configuration $configuration): array
    {}

    private function initializeTestRunHistory(Configuration $configuration): TestRunHistory
    {}

    /**
     * Pruning drops all test run history entries that the current test run
     * did not touch, so it is only safe when the current test run executes
     * every test that exists: no test selection or filtering of any kind may
     * be configured.
     */
    private function testRunHistoryMayBePruned(Configuration $configuration): bool
    {}

    private function configureBaseline(Configuration $configuration): ?BaselineGenerator
    {}

    private function checkPhpConfiguration(Configuration $configuration): void
    {}

    private function printAbortMessage(Printer $printer, Configuration $configuration, ExtensionCapabilities $extensionCapabilities, string $message): void
    {}

    /**
     * @codeCoverageIgnore
     */
    private function registerInterruptHandler(): void
    {}

    private function exitWithCrashMessage(Throwable $t): never
    {}

    private function exitWithErrorMessage(string $message): never
    {}

    /**
     * @return list<PhptTestCase|TestCase>
     */
    private function filteredTests(Configuration $configuration, TestSuite $suite): array
    {}

    private function configureDeprecationTriggers(Configuration $configuration): void
    {}

    private function configureIssueTriggerResolvers(Configuration $configuration): void
    {}

    private function configureDeprecationFilters(Configuration $configuration): void
    {}

    private function preload(): void
    {}
}
