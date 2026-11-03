<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestRunner;

use const DIRECTORY_SEPARATOR;
use const PHP_EOL;
use function array_diff_assoc;
use function array_intersect;
use function array_unique;
use function extension_loaded;
use function realpath;
use function rtrim;
use function sprintf;
use function str_starts_with;
use function xdebug_is_debugger_active;
use AssertionError;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\Framework\Assert;
use PHPUnit\Framework\AssertionFailedError;
use PHPUnit\Framework\IncompleteTestError;
use PHPUnit\Framework\ProcessIsolationException;
use PHPUnit\Framework\SkippedTest;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Api\CodeCoverage as CodeCoverageMetadataApi;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Runner\CodeCoverage;
use PHPUnit\Runner\ErrorHandler;
use PHPUnit\Runner\Exception;
use PHPUnit\Runner\ShutdownHandler;
use PHPUnit\Runner\TimeLimit\TimeLimitHandler;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use PHPUnit\TextUI\Configuration\SourceFilter;
use SebastianBergmann\CodeCoverage\Exception as CodeCoverageException;
use SebastianBergmann\CodeCoverage\InvalidArgumentException;
use SebastianBergmann\CodeCoverage\Test\Target\Target;
use SebastianBergmann\CodeCoverage\Test\Target\TargetCollection;
use SebastianBergmann\CodeCoverage\UnintentionallyCoveredCodeException;
use SebastianBergmann\Invoker\Invoker;
use SebastianBergmann\Invoker\TimeoutException;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestRunner
{
    private readonly Configuration $configuration;
    private readonly Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @throws \PHPUnit\Framework\Exception
     * @throws \PHPUnit\Util\Exception
     * @throws \SebastianBergmann\Template\InvalidArgumentException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws ProcessIsolationException
     * @throws UnintentionallyCoveredCodeException
     */
    public function run(TestCase $test): void
    {}

    /**
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws UnintentionallyCoveredCodeException
     */
    private function runInProcess(TestCase $test): void
    {}

    private function requiresCoverageMetadata(TestCase $test): bool
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    private function hasCoverageMetadata(string $className, string $methodName): bool
    {}

    private function canTimeLimitBeEnforced(): bool
    {}

    private function shouldTimeLimitBeEnforced(TestCase $test): bool
    {}

    /**
     * @throws Throwable
     */
    private function runTestWithTimeout(TestCase $test): bool
    {}

    private function shouldErrorHandlerBeUsed(TestCase $test): bool
    {}

    private function performSanityChecks(TestCase $test, TargetCollection $coversTargets, TargetCollection $usesTargets, bool $coversNothingContradiction): void
    {}

    private function removeFilesystemTargetsThatAreNotFirstPartyCode(TestCase $test, TargetCollection $targets): TargetCollection
    {}

    private function isFilesystemTargetThatIsNotFirstPartyCode(Target $target): bool
    {}

    private function isDirectoryConfiguredAsFirstPartyCode(string $directory): bool
    {}
}
