<?php declare(strict_types=1);

namespace PhpParser\Node\Scalar;

use PhpParser\Node\Expr;
use PhpParser\Node\InterpolatedStringPart;
use PhpParser\Node\Scalar;

class InterpolatedString extends Scalar {
    /** @var (Expr|InterpolatedStringPart)[] list of string parts */
    public array $parts;

    /**
     * Constructs an interpolated string node.
     *
     * @param (Expr|InterpolatedStringPart)[] $parts Interpolated string parts
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $parts, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(InterpolatedString::class, Encapsed::class);
