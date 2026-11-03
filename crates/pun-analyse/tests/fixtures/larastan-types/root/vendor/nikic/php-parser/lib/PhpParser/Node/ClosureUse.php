<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\NodeAbstract;

class ClosureUse extends NodeAbstract {
    /** @var Expr\Variable Variable to use */
    public Expr\Variable $var;
    /** @var bool Whether to use by reference */
    public bool $byRef;

    /**
     * Constructs a closure use node.
     *
     * @param Expr\Variable $var Variable to use
     * @param bool $byRef Whether to use by reference
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Expr\Variable $var, bool $byRef = false, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(ClosureUse::class, Expr\ClosureUse::class);
