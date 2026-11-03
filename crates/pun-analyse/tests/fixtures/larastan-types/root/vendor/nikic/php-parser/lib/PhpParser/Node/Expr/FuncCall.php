<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node;
use PhpParser\Node\Expr;

class FuncCall extends CallLike {
    /** @var Node\Name|Expr Function name */
    public Node $name;
    /** @var array<Node\Arg|Node\VariadicPlaceholder|Node\ArgPlaceholder> Arguments */
    public array $args;

    /**
     * Constructs a function call node.
     *
     * @param Node\Name|Expr $name Function name
     * @param array<Node\Arg|Node\VariadicPlaceholder|Node\ArgPlaceholder> $args Arguments
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Node $name, array $args = [], array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}

    public function getRawArgs(): array {}
}
