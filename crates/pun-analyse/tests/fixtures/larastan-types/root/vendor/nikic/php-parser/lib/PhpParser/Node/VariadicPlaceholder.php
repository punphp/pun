<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\NodeAbstract;

/**
 * Represents the "..." in "foo(...)" of the first-class callable syntax.
 */
class VariadicPlaceholder extends NodeAbstract {
    /**
     * Create a variadic argument placeholder (first-class callable syntax).
     *
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $attributes = []) {}

    public function getType(): string {}

    public function getSubNodeNames(): array {}
}
