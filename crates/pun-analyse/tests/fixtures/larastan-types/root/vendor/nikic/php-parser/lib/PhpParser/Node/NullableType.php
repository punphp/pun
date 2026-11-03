<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\Node;

class NullableType extends ComplexType {
    /** @var Identifier|Name Type */
    public Node $type;

    /**
     * Constructs a nullable type (wrapping another type).
     *
     * @param Identifier|Name $type Type
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(Node $type, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
