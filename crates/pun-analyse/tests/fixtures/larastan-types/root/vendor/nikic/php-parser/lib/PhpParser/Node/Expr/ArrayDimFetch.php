<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Expr;

class ArrayDimFetch extends Expr {
    /** @var Expr Variable */
    public Expr $var;
    /** @var null|Expr Array index / dim */
    public ?Expr $dim;

    /**
     * Constructs an array index fetch node.
     *
     * @param Expr $var Variable
     * @param null|Expr $dim Array index / dim
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Expr $var, ?Expr $dim = null, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
