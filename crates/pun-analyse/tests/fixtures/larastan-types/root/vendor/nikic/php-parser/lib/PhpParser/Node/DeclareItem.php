<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\Node;
use PhpParser\NodeAbstract;

class DeclareItem extends NodeAbstract {
    /** @var Node\Identifier Key */
    public Identifier $key;
    /** @var Node\Expr Value */
    public Expr $value;

    /**
     * Constructs a declare key=>value pair node.
     *
     * @param string|Node\Identifier $key Key
     * @param Node\Expr $value Value
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($key, Node\Expr $value, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(DeclareItem::class, Stmt\DeclareDeclare::class);
