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

use function array_filter;
use function count;
use SebastianBergmann\CodeCoverage\Data\ProcessedBranchCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionType;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\Data\ProcessedPathCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\StaticAnalysis\AnalysisResult;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Class_;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Function_;
use SebastianBergmann\CodeCoverage\StaticAnalysis\LinesOfCode;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Method;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Trait_;
use SebastianBergmann\CodeCoverage\Test\TestSizes;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestDataType from Builder
 * @phpstan-import-type TestIndexType from ProcessedCodeCoverageData
 * @phpstan-import-type LinesType from AnalysisResult
 * @phpstan-import-type TestSizeSet from TestSizes
 * @phpstan-import-type TestSizeCounts from TestSizes
 */
final class File extends AbstractNode
{
    /**
     * @var non-empty-string
     */
    private string $sha1;

    /**
     * @var array<positive-int, ?array<TestIndexType, positive-int>>
     */
    private array $lineCoverageData;

    /**
     * @var array<non-empty-string, ProcessedFunctionCoverageData>
     */
    private array $functionCoverageData;

    /**
     * @var array<TestIndexType, TestDataType>
     */
    private readonly array $testData;
    private int $numExecutableLines = 0;
    private int $numExecutedLines   = 0;

    /**
     * @var TestSizeCounts
     */
    private array $numExecutedLinesByTestSize = TestSizes::ZERO_COUNTS;
    private int $numExecutableBranches        = 0;
    private int $numExecutedBranches          = 0;
    private int $numExecutablePaths           = 0;
    private int $numExecutedPaths             = 0;

    /**
     * @var array<string, ProcessedClassType>
     */
    private array $classes = [];

    /**
     * @var array<string, ProcessedTraitType>
     */
    private array $traits = [];

    /**
     * @var array<string, ProcessedFunctionType>
     */
    private array $functions = [];
    private readonly LinesOfCode $linesOfCode;
    private readonly bool $hasBranchCoverageData;
    private readonly bool $collectsHitCounts;
    private ?int $numClasses      = null;
    private int $numTestedClasses = 0;

    /**
     * @var TestSizeCounts
     */
    private array $numTestedClassesByTestSize = TestSizes::ZERO_COUNTS;
    private ?int $numTraits                   = null;
    private int $numTestedTraits              = 0;

    /**
     * @var TestSizeCounts
     */
    private array $numTestedTraitsByTestSize = TestSizes::ZERO_COUNTS;
    private ?int $numMethods                 = null;
    private ?int $numTestedMethods           = null;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedMethodsByTestSize = null;
    private int $numTestedFunctions            = 0;

    /**
     * @var TestSizeCounts
     */
    private array $numTestedFunctionsByTestSize = TestSizes::ZERO_COUNTS;

    /**
     * @var array<int, array<int, ProcessedClassType|ProcessedFunctionType|ProcessedMethodType|ProcessedTraitType>>
     */
    private array $codeUnitsByLine = [];

    /**
     * @var array<string, Class_>
     */
    private readonly array $rawClasses;

    /**
     * @var array<string, Trait_>
     */
    private readonly array $rawTraits;

    /**
     * Resolves the size of each test to the bit that represents it, so that the
     * string comparisons are not repeated for every covering test of every
     * executed line. The result is the same for every file of a report, which
     * is why Builder resolves it once and passes it to every file.
     *
     * @param array<TestIndexType, TestDataType> $testData
     *
     * @return array<TestIndexType, TestSizeSet>
     */
    public static function sizeBitByTestIndex(array $testData): array
    {}

    /**
     * @param non-empty-string                                         $sha1
     * @param array<positive-int, ?array<TestIndexType, positive-int>> $lineCoverageData
     * @param array<non-empty-string, ProcessedFunctionCoverageData>   $functionCoverageData
     * @param array<TestIndexType, TestDataType>                       $testData
     * @param array<string, Class_>                                    $classes
     * @param array<string, Trait_>                                    $traits
     * @param array<string, Function_>                                 $functions
     * @param ?array<TestIndexType, TestSizeSet>                       $sizeBitByTestIndex   derived from $testData when not given
     */
    public function __construct(string $name, AbstractNode $parent, string $sha1, array $lineCoverageData, array $functionCoverageData, array $testData, array $classes, array $traits, array $functions, LinesOfCode $linesOfCode, bool $hasBranchCoverageData = false, bool $collectsHitCounts = false, ?array $sizeBitByTestIndex = null)
    {}

    public function count(): int
    {}

    /**
     * @return non-empty-string
     */
    public function sha1(): string
    {}

    /**
     * @return array<positive-int, ?array<TestIndexType, positive-int>>
     */
    public function lineCoverageData(): array
    {}

    /**
     * @return array<non-empty-string, ProcessedFunctionCoverageData>
     */
    public function functionCoverageData(): array
    {}

    /**
     * @return array<TestIndexType, TestDataType>
     */
    public function testData(): array
    {}

    /**
     * @return array<string, Class_>
     */
    public function rawClasses(): array
    {}

    /**
     * @return array<string, Trait_>
     */
    public function rawTraits(): array
    {}

    /**
     * @return array<string, ProcessedClassType>
     */
    public function classes(): array
    {}

    /**
     * @return array<string, ProcessedTraitType>
     */
    public function traits(): array
    {}

    /**
     * @return array<string, ProcessedFunctionType>
     */
    public function functions(): array
    {}

    public function linesOfCode(): LinesOfCode
    {}

    public function numberOfExecutableLines(): int
    {}

    public function numberOfExecutedLines(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfExecutedLinesByTestSize(int $testSizes): int
    {}

    public function numberOfExecutableBranches(): int
    {}

    public function numberOfExecutedBranches(): int
    {}

    public function numberOfExecutablePaths(): int
    {}

    public function numberOfExecutedPaths(): int
    {}

    public function hasBranchCoverageData(): bool
    {}

    /**
     * Whether the values in lineCoverageData() are exact execution counts (the driver that
     * collected the data counts how often a line was executed) or only mean "executed at
     * least once".
     */
    public function collectsHitCounts(): bool
    {}

    public function numberOfFilesWithoutBranchCoverageData(): int
    {}

    public function numberOfClasses(): int
    {}

    public function numberOfTestedClasses(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedClassesByTestSize(int $testSizes): int
    {}

    public function numberOfTraits(): int
    {}

    public function numberOfTestedTraits(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedTraitsByTestSize(int $testSizes): int
    {}

    public function numberOfMethods(): int
    {}

    public function numberOfTestedMethods(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedMethodsByTestSize(int $testSizes): int
    {}

    public function numberOfFunctions(): int
    {}

    public function numberOfTestedFunctions(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedFunctionsByTestSize(int $testSizes): int
    {}

    /**
     * @param array<string, Class_>             $classes
     * @param array<string, Trait_>             $traits
     * @param array<string, Function_>          $functions
     * @param array<TestIndexType, TestSizeSet> $sizeBitByTestIndex
     */
    private function calculateStatistics(array $classes, array $traits, array $functions, array $sizeBitByTestIndex): void
    {}

    /**
     * @param array<string, Class_> $classes
     */
    private function processClasses(array $classes): void
    {}

    /**
     * @param array<string, Trait_> $traits
     */
    private function processTraits(array $traits): void
    {}

    /**
     * @param array<string, Function_> $functions
     */
    private function processFunctions(array $functions): void
    {}

    private function newMethod(string $className, Method $method, string $link): ProcessedMethodType
    {}
}
