<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\Expr;
use PhpParser\Node\Name;

class ConstFetch extends Expr {
    /** @var Name Constant name */
    public Name $name;

    /**
     * Constructs a const fetch node.
     *
     * @param Name $name Constant name
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Name $name, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
