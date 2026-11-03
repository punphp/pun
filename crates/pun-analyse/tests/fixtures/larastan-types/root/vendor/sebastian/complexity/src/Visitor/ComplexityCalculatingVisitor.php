<?php declare(strict_types=1);
/*
 * This file is part of sebastian/complexity.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Complexity;

use function assert;
use function is_array;
use PhpParser\Node;
use PhpParser\Node\Expr\New_;
use PhpParser\Node\Stmt;
use PhpParser\Node\Stmt\Class_;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Function_;
use PhpParser\Node\Stmt\Interface_;
use PhpParser\Node\Stmt\Trait_;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitor;
use PhpParser\NodeVisitorAbstract;

final class ComplexityCalculatingVisitor extends NodeVisitorAbstract
{
    /**
     * @var list<Complexity>
     */
    private array $result = [];
    private bool $shortCircuitTraversal;

    public function __construct(bool $shortCircuitTraversal)
    {}

    public function enterNode(Node $node): ?int
    {}

    public function result(): ComplexityCollection
    {}

    /**
     * @param Stmt[] $statements
     *
     * @return positive-int
     */
    private function cyclomaticComplexity(array $statements): int
    {}

    /**
     * @return non-empty-string
     */
    private function classMethodName(ClassMethod $node): string
    {}

    /**
     * @return non-empty-string
     */
    private function functionName(Function_ $node): string
    {}
}
