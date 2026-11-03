<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\Node;
use PhpParser\NodeAbstract;

class MatchArm extends NodeAbstract {
    /** @var null|list<Node\Expr> */
    public ?array $conds;
    public Expr $body;

    /**
     * @param null|list<Node\Expr> $conds
     */
    public function __construct(?array $conds, Node\Expr $body, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
