<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration;

use const PHP_VERSION;
use function in_array;
use function is_dir;
use function is_file;
use function sprintf;
use function str_contains;
use function version_compare;
use PHPUnit\Event\Emitter;
use PHPUnit\Framework\Exception as FrameworkException;
use PHPUnit\Framework\TestSuite as TestSuiteObject;
use PHPUnit\Runner\Filter\CompiledGroupFilter;
use PHPUnit\Runner\TestIndex\NullTestFileSkipper;
use PHPUnit\Runner\TestIndex\TestFileSkipper;
use PHPUnit\TextUI\Configuration\TestSuiteCollection;
use PHPUnit\TextUI\RuntimeException;
use PHPUnit\TextUI\TestDirectoryNotFoundException;
use PHPUnit\TextUI\TestFileNotFoundException;
use SebastianBergmann\FileIterator\Facade;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestSuiteMapper
{
    private Emitter $emitter;
    private TestFileSkipper $skipper;

    public function __construct(Emitter $emitter, ?TestFileSkipper $skipper = null)
    {}

    /**
     * @param non-empty-string       $xmlConfigurationFile
     * @param list<non-empty-string> $includeTestSuites
     * @param list<non-empty-string> $excludeTestSuites
     * @param positive-int           $numberOfRuns
     * @param positive-int           $maxAttempts
     *
     * @throws RuntimeException
     * @throws TestDirectoryNotFoundException
     * @throws TestFileNotFoundException
     */
    public function map(string $xmlConfigurationFile, TestSuiteCollection $configuredTestSuites, array $includeTestSuites, array $excludeTestSuites, int $numberOfRuns = 1, int $maxAttempts = 1): TestSuiteObject
    {}

    /**
     * The name of a group that the --group and --exclude-group CLI options
     * parse as a conjunction of the names of other groups cannot be used to
     * select the tests in it, see CompiledGroupFilter. The group is still
     * assigned to the tests: dropping it would take them out of a group the
     * test suite is expected to have.
     *
     * @param list<non-empty-string> $groups
     * @param non-empty-string       $path
     */
    private function warnAboutGroupNamesThatCannotBeSelected(array $groups, string $path): void
    {}

    /**
     * @param array<non-empty-string, non-empty-string> $processed
     * @param non-empty-string                          $file
     * @param non-empty-string                          $testSuiteName
     */
    private function wasAlreadyAddedToAnotherTestSuite(array $processed, string $file, string $testSuiteName): bool
    {}
}
