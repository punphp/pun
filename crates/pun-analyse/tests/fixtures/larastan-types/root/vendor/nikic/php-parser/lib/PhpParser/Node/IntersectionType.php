<?php declare(strict_types=1);

namespace PhpParser\Node;

class IntersectionType extends ComplexType {
    /** @var (Identifier|Name)[] Types */
    public array $types;

    /**
     * Constructs an intersection type.
     *
     * @param (Identifier|Name)[] $types Types
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $types, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
