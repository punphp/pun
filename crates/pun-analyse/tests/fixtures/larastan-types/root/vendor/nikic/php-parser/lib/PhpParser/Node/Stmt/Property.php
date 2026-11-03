<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Modifiers;
use PhpParser\Node;
use PhpParser\Node\ComplexType;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\PropertyItem;

class Property extends Node\Stmt {
    /** @var int Modifiers */
    public int $flags;
    /** @var PropertyItem[] Properties */
    public array $props;
    /** @var null|Identifier|Name|ComplexType Type declaration */
    public ?Node $type;
    /** @var Node\AttributeGroup[] PHP attribute groups */
    public array $attrGroups;
    /** @var Node\PropertyHook[] Property hooks */
    public array $hooks;

    /**
     * Constructs a class property list node.
     *
     * @param int $flags Modifiers
     * @param PropertyItem[] $props Properties
     * @param array<string, mixed> $attributes Additional attributes
     * @param null|Identifier|Name|ComplexType $type Type declaration
     * @param Node\AttributeGroup[] $attrGroups PHP attribute groups
     * @param Node\PropertyHook[] $hooks Property hooks
     */
    public function __construct(int $flags, array $props, array $attributes = [], ?Node $type = null, array $attrGroups = [], array $hooks = []) {}

    public function getSubNodeNames(): array {}

    /**
     * Whether the property is explicitly or implicitly public.
     */
    public function isPublic(): bool {}

    /**
     * Whether the property is protected.
     */
    public function isProtected(): bool {}

    /**
     * Whether the property is private.
     */
    public function isPrivate(): bool {}

    /**
     * Whether the property is static.
     */
    public function isStatic(): bool {}

    /**
     * Whether the property is readonly.
     */
    public function isReadonly(): bool {}

    /**
     * Whether the property is abstract.
     */
    public function isAbstract(): bool {}

    /**
     * Whether the property is final.
     */
    public function isFinal(): bool {}

    /**
     * Whether the property has explicit public(set) visibility.
     */
    public function isPublicSet(): bool {}

    /**
     * Whether the property has explicit protected(set) visibility.
     */
    public function isProtectedSet(): bool {}

    /**
     * Whether the property has explicit private(set) visibility.
     */
    public function isPrivateSet(): bool {}

    public function getType(): string {}
}
