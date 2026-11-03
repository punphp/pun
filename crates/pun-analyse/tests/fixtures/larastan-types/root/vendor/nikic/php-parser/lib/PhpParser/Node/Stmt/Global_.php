<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node;

class Global_ extends Node\Stmt {
    /** @var Node\Expr[] Variables */
    public array $vars;

    /**
     * Constructs a global variables list node.
     *
     * @param Node\Expr[] $vars Variables to unset
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $vars, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
