<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\Node;
use PhpParser\NodeAbstract;

class PropertyItem extends NodeAbstract {
    /** @var Node\VarLikeIdentifier Name */
    public VarLikeIdentifier $name;
    /** @var null|Node\Expr Default */
    public ?Expr $default;

    /**
     * Constructs a class property item node.
     *
     * @param string|Node\VarLikeIdentifier $name Name
     * @param null|Node\Expr $default Default value
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, ?Node\Expr $default = null, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(PropertyItem::class, Stmt\PropertyProperty::class);
