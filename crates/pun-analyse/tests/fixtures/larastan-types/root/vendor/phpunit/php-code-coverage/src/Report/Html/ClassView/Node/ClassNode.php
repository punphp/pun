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

use function count;
use function explode;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use SebastianBergmann\CodeCoverage\Util\Percentage;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestSizeSet from TestSizes
 * @phpstan-import-type TestSizeCounts from TestSizes
 */
final class ClassNode
{
    /**
     * @var non-empty-string
     */
    private readonly string $className;
    private readonly string $namespace;

    /**
     * @var non-empty-string
     */
    private readonly string $filePath;
    private readonly int $startLine;
    private readonly int $endLine;
    private readonly ProcessedClassType $class;
    private readonly FileNode $fileNode;

    /**
     * @var list<TraitSection>
     */
    private readonly array $traitSections;

    /**
     * @var list<ParentSection>
     */
    private readonly array $parentSections;
    private readonly NamespaceNode $parent;
    private ?int $numMethods       = null;
    private ?int $numTestedMethods = null;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numExecutedLinesByTestSize = null;

    /**
     * @var ?TestSizeCounts
     */
    private ?array $numTestedMethodsByTestSize = null;

    /**
     * @param non-empty-string    $className
     * @param non-empty-string    $filePath
     * @param list<TraitSection>  $traitSections
     * @param list<ParentSection> $parentSections
     */
    public function __construct(string $className, string $namespace, string $filePath, int $startLine, int $endLine, ProcessedClassType $class, FileNode $fileNode, array $traitSections, array $parentSections, NamespaceNode $parent)
    {}

    /**
     * @return non-empty-string
     */
    public function className(): string
    {}

    public function shortName(): string
    {}

    public function namespace(): string
    {}

    /**
     * @return non-empty-string
     */
    public function filePath(): string
    {}

    public function startLine(): int
    {}

    public function endLine(): int
    {}

    public function class_(): ProcessedClassType
    {}

    public function fileNode(): FileNode
    {}

    /**
     * @return list<TraitSection>
     */
    public function traitSections(): array
    {}

    /**
     * @return list<ParentSection>
     */
    public function parentSections(): array
    {}

    public function parent(): NamespaceNode
    {}

    /**
     * @return array<string, ProcessedMethodType>
     */
    public function allMethods(): array
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
}
