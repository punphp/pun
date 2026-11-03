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

use function array_merge;
use function assert;
use function count;
use function max;
use IteratorAggregate;
use RecursiveIteratorIterator;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionType;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\StaticAnalysis\LinesOfCode;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use Traversable;

/**
 * @template-implements IteratorAggregate<int, AbstractNode>
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestSizeSet from TestSizes
 * @phpstan-import-type TestSizeCounts from TestSizes
 */
final class Directory extends AbstractNode implements IteratorAggregate
{
    /**
     * @var list<Directory|File>
     */
    private array $children = [];

    /**
     * @var list<Directory>
     */
    private array $directories = [];

    /**
     * @var list<File>
     */
    private array $files = [];

    /**
     * @var ?array<string, ProcessedClassType>
     */
    private ?array $classes = null;

    /**
     * @var ?array<string, ProcessedTraitType>
     */
    private ?array $traits = null;

    /**
     * @var ?array<string, ProcessedFunctionType>
     */
    private ?array $functions         = null;
    private ?LinesOfCode $linesOfCode = null;
    private int $numFiles             = -1;
    private int $numExecutableLines   = -1;
    private int $numExecutedLines     = -1;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numExecutedLinesByTestSize     = null;
    private int $numExecutableBranches             = -1;
    private int $numExecutedBranches               = -1;
    private int $numExecutablePaths                = -1;
    private int $numExecutedPaths                  = -1;
    private int $numFilesWithoutBranchCoverageData = -1;
    private int $numClasses                        = -1;
    private int $numTestedClasses                  = -1;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedClassesByTestSize = null;
    private int $numTraits                     = -1;
    private int $numTestedTraits               = -1;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedTraitsByTestSize = null;
    private int $numMethods                   = -1;
    private int $numTestedMethods             = -1;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedMethodsByTestSize = null;
    private int $numFunctions                  = -1;
    private int $numTestedFunctions            = -1;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedFunctionsByTestSize = null;

    /**
     * @return non-negative-int
     */
    public function count(): int
    {}

    /**
     * @return RecursiveIteratorIterator<Iterator>&Traversable<int, AbstractNode>
     */
    public function getIterator(): RecursiveIteratorIterator
    {}

    public function addDirectory(string $name): self
    {}

    public function addFile(File $file): void
    {}

    /**
     * @return list<Directory>
     */
    public function directories(): array
    {}

    /**
     * @return list<File>
     */
    public function files(): array
    {}

    /**
     * @return list<Directory|File>
     */
    public function children(): array
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

    /**
     * @return non-negative-int
     */
    public function numberOfExecutableLines(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutedLines(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfExecutedLinesByTestSize(int $testSizes): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutableBranches(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutedBranches(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutablePaths(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutedPaths(): int
    {}

    /**
     * @return non-negative-int
     */
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
}
