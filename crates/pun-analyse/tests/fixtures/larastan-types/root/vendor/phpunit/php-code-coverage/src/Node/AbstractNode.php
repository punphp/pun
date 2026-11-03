<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Node;

use const DIRECTORY_SEPARATOR;
use function array_merge;
use function max;
use function str_ends_with;
use function str_replace;
use function substr;
use Countable;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionType;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\StaticAnalysis\LinesOfCode;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use SebastianBergmann\CodeCoverage\Util\Percentage;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestSizeSet from TestSizes
 */
abstract class AbstractNode implements Countable
{
    private readonly string $name;
    private string $pathAsString;

    /**
     * @var non-empty-list<self>
     */
    private array $pathAsArray;
    private readonly ?AbstractNode $parent;
    private string $id;

    public function __construct(string $name, ?self $parent = null)
    {}

    public function name(): string
    {}

    public function id(): string
    {}

    public function pathAsString(): string
    {}

    /**
     * @return non-empty-list<self>
     */
    public function pathAsArray(): array
    {}

    public function parent(): ?self
    {}

    public function percentageOfTestedClasses(): Percentage
    {}

    public function percentageOfTestedTraits(): Percentage
    {}

    public function percentageOfTestedClassesAndTraits(): Percentage
    {}

    public function percentageOfTestedFunctions(): Percentage
    {}

    public function percentageOfTestedMethods(): Percentage
    {}

    public function percentageOfTestedFunctionsAndMethods(): Percentage
    {}

    public function percentageOfExecutedLines(): Percentage
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function percentageOfExecutedLinesByTestSize(int $testSizes): Percentage
    {}

    public function percentageOfExecutedBranches(): Percentage
    {}

    public function percentageOfExecutedPaths(): Percentage
    {}

    public function numberOfClassesAndTraits(): int
    {}

    public function numberOfTestedClassesAndTraits(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedClassesAndTraitsByTestSize(int $testSizes): int
    {}

    /**
     * @return array<string, ProcessedClassType|ProcessedTraitType>
     */
    public function classesAndTraits(): array
    {}

    public function numberOfFunctionsAndMethods(): int
    {}

    public function numberOfTestedFunctionsAndMethods(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedFunctionsAndMethodsByTestSize(int $testSizes): int
    {}

    /**
     * @return non-negative-int
     */
    public function cyclomaticComplexity(): int
    {}

    /**
     * @return array<string, ProcessedClassType>
     */
    abstract public function classes(): array;

    /**
     * @return array<string, ProcessedTraitType>
     */
    abstract public function traits(): array;

    /**
     * @return array<string, ProcessedFunctionType>
     */
    abstract public function functions(): array;

    abstract public function linesOfCode(): LinesOfCode;

    abstract public function numberOfExecutableLines(): int;

    abstract public function numberOfExecutedLines(): int;

    /**
     * @param TestSizeSet $testSizes
     */
    abstract public function numberOfExecutedLinesByTestSize(int $testSizes): int;

    abstract public function numberOfExecutableBranches(): int;

    abstract public function numberOfExecutedBranches(): int;

    abstract public function numberOfExecutablePaths(): int;

    abstract public function numberOfExecutedPaths(): int;

    abstract public function numberOfFilesWithoutBranchCoverageData(): int;

    abstract public function numberOfClasses(): int;

    abstract public function numberOfTestedClasses(): int;

    /**
     * @param TestSizeSet $testSizes
     */
    abstract public function numberOfTestedClassesByTestSize(int $testSizes): int;

    abstract public function numberOfTraits(): int;

    abstract public function numberOfTestedTraits(): int;

    /**
     * @param TestSizeSet $testSizes
     */
    abstract public function numberOfTestedTraitsByTestSize(int $testSizes): int;

    abstract public function numberOfMethods(): int;

    abstract public function numberOfTestedMethods(): int;

    /**
     * @param TestSizeSet $testSizes
     */
    abstract public function numberOfTestedMethodsByTestSize(int $testSizes): int;

    abstract public function numberOfFunctions(): int;

    abstract public function numberOfTestedFunctions(): int;

    /**
     * @param TestSizeSet $testSizes
     */
    abstract public function numberOfTestedFunctionsByTestSize(int $testSizes): int;

    private function processId(): void
    {}

    private function processPath(): void
    {}
}
