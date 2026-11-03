<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Expr;

class Empty_ extends Expr {
    /** @var Expr Expression */
    public Expr $expr;

    /**
     * Constructs an empty() node.
     *
     * @param Expr $expr Expression
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Expr $expr, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
