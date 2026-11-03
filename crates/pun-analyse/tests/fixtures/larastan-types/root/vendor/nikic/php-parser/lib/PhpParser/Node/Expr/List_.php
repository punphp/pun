<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node\ArrayItem;
use PhpParser\Node\Expr;

class List_ extends Expr {
    // For use in "kind" attribute
    public const KIND_LIST = 1; // list() syntax
    public const KIND_ARRAY = 2; // [] syntax

    /** @var (ArrayItem|null)[] List of items to assign to */
    public array $items;

    /**
     * Constructs a list() destructuring node.
     *
     * @param (ArrayItem|null)[] $items List of items to assign to
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $items, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
