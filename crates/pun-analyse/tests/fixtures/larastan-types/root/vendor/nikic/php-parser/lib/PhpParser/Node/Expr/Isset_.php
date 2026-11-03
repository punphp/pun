<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Expr;

class Isset_ extends Expr {
    /** @var Expr[] Variables */
    public array $vars;

    /**
     * Constructs an array node.
     *
     * @param Expr[] $vars Variables
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $vars, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
