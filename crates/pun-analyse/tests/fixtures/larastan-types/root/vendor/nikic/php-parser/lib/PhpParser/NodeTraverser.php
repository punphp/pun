<?php declare(strict_types=1);

namespace PhpParser;

class NodeTraverser implements NodeTraverserInterface {
    /**
     * @deprecated Use NodeVisitor::DONT_TRAVERSE_CHILDREN instead.
     */
    public const DONT_TRAVERSE_CHILDREN = NodeVisitor::DONT_TRAVERSE_CHILDREN;

    /**
     * @deprecated Use NodeVisitor::STOP_TRAVERSAL instead.
     */
    public const STOP_TRAVERSAL = NodeVisitor::STOP_TRAVERSAL;

    /**
     * @deprecated Use NodeVisitor::REMOVE_NODE instead.
     */
    public const REMOVE_NODE = NodeVisitor::REMOVE_NODE;

    /**
     * @deprecated Use NodeVisitor::DONT_TRAVERSE_CURRENT_AND_CHILDREN instead.
     */
    public const DONT_TRAVERSE_CURRENT_AND_CHILDREN = NodeVisitor::DONT_TRAVERSE_CURRENT_AND_CHILDREN;

    /** @var list<NodeVisitor> Visitors */
    protected array $visitors = [];

    /** @var bool Whether traversal should be stopped */
    protected bool $stopTraversal;

    /**
     * Create a traverser with the given visitors.
     *
     * @param NodeVisitor ...$visitors Node visitors
     */
    public function __construct(NodeVisitor ...$visitors) {}

    /**
     * Adds a visitor.
     *
     * @param NodeVisitor $visitor Visitor to add
     */
    public function addVisitor(NodeVisitor $visitor): void {}

    /**
     * Removes an added visitor.
     */
    public function removeVisitor(NodeVisitor $visitor): void {}

    /**
     * Traverses an array of nodes using the registered visitors.
     *
     * @param Node[] $nodes Array of nodes
     *
     * @return Node[] Traversed array of nodes
     */
    public function traverse(array $nodes): array {}

    /**
     * Recursively traverse a node.
     *
     * @param Node $node Node to traverse.
     */
    protected function traverseNode(Node $node): void {}

    /**
     * Recursively traverse array (usually of nodes).
     *
     * @param Node[] $nodes Array to traverse
     *
     * @return Node[] Result of traversal (may be original array or changed one)
     */
    protected function traverseArray(array $nodes): array {}

    private function ensureReplacementReasonable(Node $old, Node $new): void {}
}
