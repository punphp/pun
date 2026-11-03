<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Expr;

class YieldFrom extends Expr {
    /** @var Expr Expression to yield from */
    public Expr $expr;

    /**
     * Constructs an "yield from" node.
     *
     * @param Expr $expr Expression
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Expr $expr, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
