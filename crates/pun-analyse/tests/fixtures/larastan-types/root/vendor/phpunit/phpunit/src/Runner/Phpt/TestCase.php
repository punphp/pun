<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Phpt;

use const DEBUG_BACKTRACE_IGNORE_ARGS;
use const DIRECTORY_SEPARATOR;
use function array_filter;
use function array_values;
use function assert;
use function bin2hex;
use function debug_backtrace;
use function dirname;
use function explode;
use function extension_loaded;
use function file_get_contents;
use function getenv;
use function in_array;
use function is_array;
use function is_file;
use function is_string;
use function ltrim;
use function mkdir;
use function ob_get_clean;
use function ob_start;
use function preg_match;
use function preg_replace;
use function preg_replace_callback;
use function random_bytes;
use function realpath;
use function rmdir;
use function sprintf;
use function str_contains;
use function str_replace;
use function str_starts_with;
use function strncasecmp;
use function substr;
use function sys_get_temp_dir;
use function trim;
use function unlink;
use function unserialize;
use PHPUnit\Event\Code\Phpt;
use PHPUnit\Event\Code\ThrowableBuilder;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\Event\TestRunner\ChildProcessReason;
use PHPUnit\Framework\Assert;
use PHPUnit\Framework\AssertionFailedError;
use PHPUnit\Framework\ExecutionOrderDependency;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\IncompleteTestError;
use PHPUnit\Framework\PhptAssertionFailedError;
use PHPUnit\Framework\Reorderable;
use PHPUnit\Framework\SelfDescribing;
use PHPUnit\Framework\Test;
use PHPUnit\Runner\CodeCoverage;
use PHPUnit\Runner\Exception;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use PHPUnit\Util\PHP\Job;
use PHPUnit\Util\PHP\JobRunnerRegistry;
use SebastianBergmann\CodeCoverage\Data\RawCodeCoverageData;
use SebastianBergmann\CodeCoverage\InvalidArgumentException;
use SebastianBergmann\CodeCoverage\ReflectionException;
use SebastianBergmann\CodeCoverage\Test\TestSize;
use SebastianBergmann\CodeCoverage\Test\TestStatus;
use SebastianBergmann\CodeCoverage\TestIdMissingException;
use SebastianBergmann\CodeCoverage\UnintentionallyCoveredCodeException;
use staabm\SideEffectsDetector\SideEffect;
use staabm\SideEffectsDetector\SideEffectsDetector;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @see https://qa.php.net/phpt_details.php
 *
 * @phpstan-type CoverageFiles array{coverage: non-empty-string, job: non-empty-string}
 */
final readonly class TestCase implements Reorderable, SelfDescribing, Test
{
    /**
     * @var non-empty-string
     */
    private string $filename;

    /**
     * @var positive-int
     */
    private int $repetition;

    /**
     * @var positive-int
     */
    private int $totalRepetitions;

    /**
     * @var positive-int
     */
    private int $attempt;

    /**
     * @var positive-int
     */
    private int $maxAttempts;

    /**
     * The directory the temporary files that are used to collect code coverage
     * for this test are created in.
     *
     * @var non-empty-string
     */
    private string $coverageFilesDirectory;

    /**
     * @var CoverageFiles
     */
    private array $coverageFiles;

    /**
     * @param non-empty-string $filename
     * @param positive-int     $repetition
     * @param positive-int     $totalRepetitions
     * @param positive-int     $attempt
     * @param positive-int     $maxAttempts
     */
    public function __construct(string $filename, int $repetition = 1, int $totalRepetitions = 1, int $attempt = 1, int $maxAttempts = 1)
    {}

    public function count(): int
    {}

    /**
     * @throws \PHPUnit\Framework\Exception
     * @throws \SebastianBergmann\Template\InvalidArgumentException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws ReflectionException
     * @throws TestIdMissingException
     * @throws UnintentionallyCoveredCodeException
     */
    public function run(): void
    {}

    /**
     * Returns the name of the test case.
     */
    public function getName(): string
    {}

    /**
     * Returns a string representation of the test case.
     */
    public function toString(): string
    {}

    /**
     * The file that declares this test.
     *
     * @return non-empty-string
     */
    public function file(): string
    {}

    public function sortId(): string
    {}

    /**
     * @return list<ExecutionOrderDependency>
     */
    public function provides(): array
    {}

    /**
     * @return list<ExecutionOrderDependency>
     */
    public function requires(): array
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function valueObjectForEvents(): Phpt
    {}

    /**
     * @return positive-int
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function repetition(): int
    {}

    /**
     * @param array<non-empty-string, string> $sections
     *
     * @throws ExpectationFailedException
     */
    private function assertPhptExpectation(array $sections, string $output): void
    {}

    /**
     * A SKIPIF section that prints "xfail <reason>" declares an expected
     * failure at runtime; it is treated as if the PHPT file contained an
     * XFAIL section with that reason.
     *
     * @param array<non-empty-string, string>               $sections
     * @param array<non-empty-string, array<string>|string> $settings
     */
    private function shouldTestBeSkipped(array &$sections, array $settings, Emitter $emitter): bool
    {}

    /**
     * @param array<non-empty-string, string> $sections
     */
    private function shouldRunInSubprocess(array $sections, string $cleanCode): bool
    {}

    private function runCodeInLocalSandbox(string $code): string
    {}

    /**
     * @param array<non-empty-string, string> $sections
     */
    private function runClean(array $sections, bool $collectCoverage, Emitter $emitter): void
    {}

    /**
     * @codeCoverageIgnore
     *
     * @phpstan-ignore return.internalClass
     */
    private function cleanupForCoverage(Emitter $emitter): RawCodeCoverageData
    {}

    /**
     * @param array<non-empty-string, array<string>|string> $ini
     *
     * @return list<non-empty-string>
     */
    private function stringifyIni(array $ini): array
    {}

    /**
     * @param array<non-empty-string, string> $sections
     *
     * @return non-empty-list<array{file: non-empty-string, line: int}>
     */
    private function locationHintFromDiff(string $message, array $sections): array
    {}

    private function cleanDiffLine(string $line): string
    {}

    /**
     * @param array<non-empty-string, string> $sections
     *
     * @return non-empty-list<array{file: non-empty-string, line: int}>
     */
    private function locationHint(string $needle, array $sections): array
    {}

    /**
     * @return list<string>
     */
    private function settings(bool $collectCoverage): array
    {}

    private function triggerRunnerWarningOnPhpErrors(string $section, string $output, Emitter $emitter): void
    {}

    /**
     * @param array<non-empty-string, string> $sections
     * @param non-empty-string                $key
     */
    private function sectionOffset(array $sections, string $key): int
    {}
}
