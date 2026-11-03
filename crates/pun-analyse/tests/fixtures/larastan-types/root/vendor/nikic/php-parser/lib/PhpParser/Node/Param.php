<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\Modifiers;
use PhpParser\Node;
use PhpParser\NodeAbstract;

class Param extends NodeAbstract {
    /** @var null|Identifier|Name|ComplexType Type declaration */
    public ?Node $type;
    /** @var bool Whether parameter is passed by reference */
    public bool $byRef;
    /** @var bool Whether this is a variadic argument */
    public bool $variadic;
    /** @var Expr\Variable|Expr\Error Parameter variable */
    public Expr $var;
    /** @var null|Expr Default value */
    public ?Expr $default;
    /** @var int Optional visibility flags */
    public int $flags;
    /** @var AttributeGroup[] PHP attribute groups */
    public array $attrGroups;
    /** @var PropertyHook[] Property hooks for promoted properties */
    public array $hooks;

    /**
     * Constructs a parameter node.
     *
     * @param Expr\Variable|Expr\Error $var Parameter variable
     * @param null|Expr $default Default value
     * @param null|Identifier|Name|ComplexType $type Type declaration
     * @param bool $byRef Whether is passed by reference
     * @param bool $variadic Whether this is a variadic argument
     * @param array<string, mixed> $attributes Additional attributes
     * @param int $flags Optional visibility flags
     * @param list<AttributeGroup> $attrGroups PHP attribute groups
     * @param PropertyHook[] $hooks Property hooks for promoted properties
     */
    public function __construct(
        Expr $var, ?Expr $default = null, ?Node $type = null,
        bool $byRef = false, bool $variadic = false,
        array $attributes = [],
        int $flags = 0,
        array $attrGroups = [],
        array $hooks = []
    ) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}

    /**
     * Whether this parameter uses constructor property promotion.
     */
    public function isPromoted(): bool {}

    public function isFinal(): bool {}

    public function isPublic(): bool {}

    public function isProtected(): bool {}

    public function isPrivate(): bool {}

    public function isReadonly(): bool {}

    /**
     * Whether the promoted property has explicit public(set) visibility.
     */
    public function isPublicSet(): bool {}

    /**
     * Whether the promoted property has explicit protected(set) visibility.
     */
    public function isProtectedSet(): bool {}

    /**
     * Whether the promoted property has explicit private(set) visibility.
     */
    public function isPrivateSet(): bool {}
}
