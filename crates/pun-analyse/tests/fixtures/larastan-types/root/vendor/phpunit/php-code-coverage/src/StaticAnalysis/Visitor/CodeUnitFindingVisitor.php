<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\StaticAnalysis;

use function assert;
use function implode;
use function is_string;
use function max;
use function strlen;
use function substr;
use PhpParser\Node;
use PhpParser\Node\ComplexType;
use PhpParser\Node\Expr\Variable;
use PhpParser\Node\Identifier;
use PhpParser\Node\IntersectionType;
use PhpParser\Node\Name;
use PhpParser\Node\NullableType;
use PhpParser\Node\Stmt\Class_ as PhpParserClass_;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Enum_ as PhpParserEnum;
use PhpParser\Node\Stmt\Function_ as PhpParserFunction_;
use PhpParser\Node\Stmt\Interface_ as PhpParserInterface_;
use PhpParser\Node\Stmt\Trait_ as PhpParserTrait_;
use PhpParser\Node\UnionType;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitorAbstract;
use SebastianBergmann\Complexity\CyclomaticComplexityCalculatingVisitor;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class CodeUnitFindingVisitor extends NodeVisitorAbstract
{
    /**
     * @var non-empty-string
     */
    private string $file;

    /**
     * @var array<string, Interface_>
     */
    private array $interfaces = [];

    /**
     * @var array<string, Class_>
     */
    private array $classes = [];

    /**
     * @var array<string, Trait_>
     */
    private array $traits = [];

    /**
     * @var array<string, Function_>
     */
    private array $functions = [];

    /**
     * @param non-empty-string $file
     */
    public function __construct(string $file)
    {}

    public function enterNode(Node $node): null
    {}

    public function leaveNode(Node $node): null
    {}

    /**
     * @return array<string, Interface_>
     */
    public function interfaces(): array
    {}

    /**
     * @return array<string, Class_>
     */
    public function classes(): array
    {}

    /**
     * @return array<string, Trait_>
     */
    public function traits(): array
    {}

    /**
     * @return array<string, Function_>
     */
    public function functions(): array
    {}

    /**
     * @return non-negative-int
     */
    private function cyclomaticComplexity(ClassMethod|PhpParserFunction_ $node): int
    {}

    /**
     * @return non-empty-string
     */
    private function signature(ClassMethod|PhpParserFunction_ $node): string
    {}

    private function type(ComplexType|Identifier|Name $type): string
    {}

    private function visibility(ClassMethod $node): Visibility
    {}

    private function processInterface(PhpParserInterface_ $node): void
    {}

    private function processClass(PhpParserClass_|PhpParserEnum $node): void
    {}

    private function processTrait(PhpParserTrait_ $node): void
    {}

    /**
     * @return list<non-empty-string>
     */
    private function traitUses(PhpParserClass_|PhpParserEnum|PhpParserTrait_ $node): array
    {}

    /**
     * @param list<ClassMethod> $nodes
     *
     * @return array<non-empty-string, Method>
     */
    private function processMethods(array $nodes): array
    {}

    private function processFunction(PhpParserFunction_ $node): void
    {}

    /**
     * @return positive-int
     */
    private function startLine(ClassMethod|PhpParserClass_|PhpParserEnum|PhpParserFunction_|PhpParserInterface_|PhpParserTrait_ $node): int
    {}

    /**
     * @return positive-int
     */
    private function endLine(ClassMethod|PhpParserClass_|PhpParserEnum|PhpParserFunction_|PhpParserInterface_|PhpParserTrait_ $node): int
    {}

    private function namespace(string $namespacedName, string $name): string
    {}

    private function unionTypeAsString(UnionType $node): string
    {}

    private function intersectionTypeAsString(IntersectionType $node): string
    {}

    private function typeAsString(Identifier|Name $node): string
    {}

    /**
     * @param list<non-empty-string> $traits
     */
    private function setTraits(PhpParserClass_|PhpParserEnum|PhpParserTrait_ $node, array $traits): void
    {}
}
