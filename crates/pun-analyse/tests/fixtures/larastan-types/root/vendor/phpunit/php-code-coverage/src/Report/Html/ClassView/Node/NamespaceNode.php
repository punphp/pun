<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node;

use function array_merge;
use function str_replace;
use Generator;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use SebastianBergmann\CodeCoverage\Util\Percentage;

/**
 * Aggregated metrics count the own code of each class in the namespace exactly once and each
 * trait used by those classes exactly once, even when a trait is used by multiple classes;
 * code inherited from a parent class is counted at the node of the class that declares it.
 * ClassNode metrics, in contrast, include the code a class uses from traits and inherits from
 * parent classes, so the metrics of the classes in a namespace may add up to more than the
 * metrics of the namespace itself.
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestSizeSet from TestSizes
 * @phpstan-import-type TestSizeCounts from TestSizes
 */
final class NamespaceNode
{
    private readonly string $name;
    private readonly string $namespace;
    private ?self $parent;

    /**
     * @var list<self>
     */
    private array $childNamespaces = [];

    /**
     * @var list<ClassNode>
     */
    private array $classes             = [];
    private int $numExecutableLines    = -1;
    private int $numExecutedLines      = -1;
    private int $numExecutableBranches = -1;
    private int $numExecutedBranches   = -1;
    private int $numExecutablePaths    = -1;
    private int $numExecutedPaths      = -1;
    private int $numClasses            = -1;
    private int $numTestedClasses      = -1;
    private int $numMethods            = -1;
    private int $numTestedMethods      = -1;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numExecutedLinesByTestSize = null;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedClassesByTestSize = null;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedMethodsByTestSize = null;

    public function __construct(string $name, string $namespace, ?self $parent = null)
    {}

    public function name(): string
    {}

    public function namespace(): string
    {}

    public function parent(): ?self
    {}

    public function promoteToRoot(): void
    {}

    public function id(): string
    {}

    /**
     * @return non-empty-list<self>
     */
    public function pathAsArray(): array
    {}

    public function addNamespace(self $namespace): void
    {}

    public function addClass(ClassNode $class): void
    {}

    /**
     * @return list<self>
     */
    public function childNamespaces(): array
    {}

    /**
     * @return list<ClassNode>
     */
    public function classes(): array
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

    public function numberOfClasses(): int
    {}

    public function numberOfTestedClasses(): int
    {}

    /**
     * @param TestSizeSet $testSizes
     */
    public function numberOfTestedClassesByTestSize(int $testSizes): int
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

    public function percentageOfExecutedLines(): Percentage
    {}

    public function percentageOfExecutedBranches(): Percentage
    {}

    public function percentageOfExecutedPaths(): Percentage
    {}

    public function percentageOfTestedMethods(): Percentage
    {}

    public function percentageOfTestedClasses(): Percentage
    {}

    /**
     * @return array<string, ProcessedClassType>
     */
    public function allClassTypes(): array
    {}

    /**
     * Yields all ClassNode and NamespaceNode descendants (depth-first).
     *
     * @return Generator<ClassNode|self>
     */
    public function iterate(): Generator
    { yield; }

    /**
     * @return list<ClassNode>
     */
    private function classesInSubtree(): array
    {}

    /**
     * @return array<non-empty-string, ProcessedTraitType>
     */
    private function traitsInSubtree(): array
    {}

    /**
     * @return list<ProcessedMethodType>
     */
    private function methodsInSubtree(): array
    {}

    private function resetCounters(): void
    {}
}
