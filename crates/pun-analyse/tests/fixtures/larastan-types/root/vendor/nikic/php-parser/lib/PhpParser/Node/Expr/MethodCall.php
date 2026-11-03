<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\ArgPlaceholder;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\VariadicPlaceholder;

class MethodCall extends CallLike {
    /** @var Expr Variable holding object */
    public Expr $var;
    /** @var Identifier|Expr Method name */
    public Node $name;
    /** @var array<Arg|VariadicPlaceholder|ArgPlaceholder> Arguments */
    public array $args;

    /**
     * Constructs a function call node.
     *
     * @param Expr $var Variable holding object
     * @param string|Identifier|Expr $name Method name
     * @param array<Arg|VariadicPlaceholder|ArgPlaceholder> $args Arguments
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Expr $var, $name, array $args = [], array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}

    public function getRawArgs(): array {}
}
