<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Modifiers;
use PhpParser\Node;

class ClassConst extends Node\Stmt {
    /** @var int Modifiers */
    public int $flags;
    /** @var Node\Const_[] Constant declarations */
    public array $consts;
    /** @var Node\AttributeGroup[] PHP attribute groups */
    public array $attrGroups;
    /** @var Node\Identifier|Node\Name|Node\ComplexType|null Type declaration */
    public ?Node $type;

    /**
     * Constructs a class const list node.
     *
     * @param Node\Const_[] $consts Constant declarations
     * @param int $flags Modifiers
     * @param array<string, mixed> $attributes Additional attributes
     * @param list<Node\AttributeGroup> $attrGroups PHP attribute groups
     * @param null|Node\Identifier|Node\Name|Node\ComplexType $type Type declaration
     */
    public function __construct(
        array $consts,
        int $flags = 0,
        array $attributes = [],
        array $attrGroups = [],
        ?Node $type = null
    ) {}

    public function getSubNodeNames(): array {}

    /**
     * Whether constant is explicitly or implicitly public.
     */
    public function isPublic(): bool {}

    /**
     * Whether constant is protected.
     */
    public function isProtected(): bool {}

    /**
     * Whether constant is private.
     */
    public function isPrivate(): bool {}

    /**
     * Whether constant is final.
     */
    public function isFinal(): bool {}

    public function getType(): string {}
}
