<?php declare(strict_types=1);

namespace PhpParser\NodeVisitor;

use PhpParser\ErrorHandler;
use PhpParser\NameContext;
use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Name;
use PhpParser\Node\Name\FullyQualified;
use PhpParser\Node\Stmt;
use PhpParser\NodeVisitorAbstract;

class NameResolver extends NodeVisitorAbstract {
    /** @var NameContext Naming context */
    protected NameContext $nameContext;

    /** @var bool Whether to preserve original names */
    protected bool $preserveOriginalNames;

    /** @var bool Whether to replace resolved nodes in place, or to add resolvedNode attributes */
    protected bool $replaceNodes;

    /**
     * Constructs a name resolution visitor.
     *
     * Options:
     *  * preserveOriginalNames (default false): An "originalName" attribute will be added to
     *    all name nodes that underwent resolution.
     *  * replaceNodes (default true): Resolved names are replaced in-place. Otherwise, a
     *    resolvedName attribute is added. (Names that cannot be statically resolved receive a
     *    namespacedName attribute, as usual.)
     *
     * @param ErrorHandler|null $errorHandler Error handler
     * @param array{preserveOriginalNames?: bool, replaceNodes?: bool} $options Options
     */
    public function __construct(?ErrorHandler $errorHandler = null, array $options = []) {}

    /**
     * Get name resolution context.
     */
    public function getNameContext(): NameContext {}

    public function beforeTraverse(array $nodes): ?array {}

    public function enterNode(Node $node) {}

    /** @param Stmt\Use_::TYPE_* $type */
    private function addAlias(Node\UseItem $use, int $type, ?Name $prefix = null): void {}

    /** @param Stmt\Function_|Stmt\ClassMethod|Expr\Closure|Expr\ArrowFunction $node */
    private function resolveSignature($node): void {}

    /**
     * @template T of Node\Identifier|Name|Node\ComplexType|null
     * @param T $node
     * @return T
     */
    private function resolveType(?Node $node): ?Node {}

    /**
     * Resolve name, according to name resolver options.
     *
     * @param Name $name Function or constant name to resolve
     * @param Stmt\Use_::TYPE_* $type One of Stmt\Use_::TYPE_*
     *
     * @return Name Resolved name, or original name with attribute
     */
    protected function resolveName(Name $name, int $type): Name {}

    protected function resolveClassName(Name $name): Name {}

    protected function addNamespacedName(Node $node): void {}

    protected function resolveAttrGroups(Node $node): void {}
}
