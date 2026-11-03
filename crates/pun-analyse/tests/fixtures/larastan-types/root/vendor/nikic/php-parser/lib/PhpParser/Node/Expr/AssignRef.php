<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Expr;

class AssignRef extends Expr {
    /** @var Expr Variable reference is assigned to */
    public Expr $var;
    /** @var Expr Variable which is referenced */
    public Expr $expr;

    /**
     * Constructs an assignment node.
     *
     * @param Expr $var Variable
     * @param Expr $expr Expression
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Expr $var, Expr $expr, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
