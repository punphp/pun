<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use const DIRECTORY_SEPARATOR;
use const PHP_EOL;
use function assert;
use function count;
use function dirname;
use function file;
use function is_dir;
use function is_file;
use function realpath;
use function str_ends_with;
use function trim;
use PHPUnit\Event\Emitter;
use PHPUnit\Exception;
use PHPUnit\Framework\TestSuite;
use PHPUnit\Runner\TestIndex\NullTestFileSkipper;
use PHPUnit\Runner\TestIndex\TestFileSkipper;
use PHPUnit\Runner\TestSuiteLoader;
use PHPUnit\TextUI\RuntimeException;
use PHPUnit\TextUI\TestDirectoryNotFoundException;
use PHPUnit\TextUI\TestFileNotFoundException;
use PHPUnit\TextUI\XmlConfiguration\TestSuiteMapper;
use SebastianBergmann\FileIterator\Facade as FileIteratorFacade;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestSuiteBuilder
{
    private Emitter $emitter;
    private TestFileSkipper $skipper;

    public function __construct(Emitter $emitter, ?TestFileSkipper $skipper = null)
    {}

    /**
     * @throws \PHPUnit\Framework\Exception
     * @throws RuntimeException
     * @throws TestDirectoryNotFoundException
     * @throws TestFileNotFoundException
     */
    public function build(Configuration $configuration): TestSuite
    {}

    /**
     * @param non-empty-string       $path
     * @param list<non-empty-string> $suffixes
     * @param positive-int           $numberOfRuns
     * @param positive-int           $maxAttempts
     *
     * @throws \PHPUnit\Framework\Exception
     */
    private function testSuiteFromPath(string $path, array $suffixes, int $numberOfRuns, int $maxAttempts, ?TestSuite $suite = null): TestSuite
    {}

    /**
     * @param list<non-empty-string> $paths
     * @param list<non-empty-string> $suffixes
     * @param positive-int           $numberOfRuns
     * @param positive-int           $maxAttempts
     *
     * @throws \PHPUnit\Framework\Exception
     */
    private function testSuiteFromPathList(array $paths, array $suffixes, int $numberOfRuns, int $maxAttempts): TestSuite
    {}
}
