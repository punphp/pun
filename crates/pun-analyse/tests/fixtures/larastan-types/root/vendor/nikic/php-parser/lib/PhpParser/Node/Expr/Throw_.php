<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node;

class Throw_ extends Node\Expr {
    /** @var Node\Expr Expression */
    public Node\Expr $expr;

    /**
     * Constructs a throw expression node.
     *
     * @param Node\Expr $expr Expression
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Node\Expr $expr, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
