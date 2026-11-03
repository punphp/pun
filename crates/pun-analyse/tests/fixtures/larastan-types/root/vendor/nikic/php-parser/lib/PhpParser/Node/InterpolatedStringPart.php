<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\NodeAbstract;

class InterpolatedStringPart extends NodeAbstract {
    /** @var string String value */
    public string $value;

    /**
     * Constructs a node representing a string part of an interpolated string.
     *
     * @param string $value String value
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(string $value, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(InterpolatedStringPart::class, Scalar\EncapsedStringPart::class);
