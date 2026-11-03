<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node;

class Echo_ extends Node\Stmt {
    /** @var Node\Expr[] Expressions */
    public array $exprs;

    /**
     * Constructs an echo node.
     *
     * @param Node\Expr[] $exprs Expressions
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $exprs, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
