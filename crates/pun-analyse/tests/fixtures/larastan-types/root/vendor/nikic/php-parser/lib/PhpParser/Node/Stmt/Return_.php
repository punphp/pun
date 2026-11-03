<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node;

class Return_ extends Node\Stmt {
    /** @var null|Node\Expr Expression */
    public ?Node\Expr $expr;

    /**
     * Constructs a return node.
     *
     * @param null|Node\Expr $expr Expression
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(?Node\Expr $expr = null, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
