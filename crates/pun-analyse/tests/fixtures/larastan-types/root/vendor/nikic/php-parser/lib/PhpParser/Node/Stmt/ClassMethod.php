<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Modifiers;
use PhpParser\Node;
use PhpParser\Node\FunctionLike;

class ClassMethod extends Node\Stmt implements FunctionLike {
    /** @var int Flags */
    public int $flags;
    /** @var bool Whether to return by reference */
    public bool $byRef;
    /** @var Node\Identifier Name */
    public Node\Identifier $name;
    /** @var Node\Param[] Parameters */
    public array $params;
    /** @var null|Node\Identifier|Node\Name|Node\ComplexType Return type */
    public ?Node $returnType;
    /** @var Node\Stmt[]|null Statements */
    public ?array $stmts;
    /** @var Node\AttributeGroup[] PHP attribute groups */
    public array $attrGroups;

    /** @var array<string, bool> */
    private static array $magicNames = [
        '__construct'   => true,
        '__destruct'    => true,
        '__call'        => true,
        '__callstatic'  => true,
        '__get'         => true,
        '__set'         => true,
        '__isset'       => true,
        '__unset'       => true,
        '__sleep'       => true,
        '__wakeup'      => true,
        '__tostring'    => true,
        '__set_state'   => true,
        '__clone'       => true,
        '__invoke'      => true,
        '__debuginfo'   => true,
        '__serialize'   => true,
        '__unserialize' => true,
    ];

    /**
     * Constructs a class method node.
     *
     * @param string|Node\Identifier $name Name
     * @param array{
     *     flags?: int,
     *     byRef?: bool,
     *     params?: Node\Param[],
     *     returnType?: null|Node\Identifier|Node\Name|Node\ComplexType,
     *     stmts?: Node\Stmt[]|null,
     *     attrGroups?: Node\AttributeGroup[],
     * } $subNodes Array of the following optional subnodes:
     *             'flags       => 0              : Flags
     *             'byRef'      => false          : Whether to return by reference
     *             'params'     => array()        : Parameters
     *             'returnType' => null           : Return type
     *             'stmts'      => array()        : Statements
     *             'attrGroups' => array()        : PHP attribute groups
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, array $subNodes = [], array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function returnsByRef(): bool {}

    public function getParams(): array {}

    public function getReturnType() {}

    public function getStmts(): ?array {}

    public function getAttrGroups(): array {}

    /**
     * Whether the method is explicitly or implicitly public.
     */
    public function isPublic(): bool {}

    /**
     * Whether the method is protected.
     */
    public function isProtected(): bool {}

    /**
     * Whether the method is private.
     */
    public function isPrivate(): bool {}

    /**
     * Whether the method is abstract.
     */
    public function isAbstract(): bool {}

    /**
     * Whether the method is final.
     */
    public function isFinal(): bool {}

    /**
     * Whether the method is static.
     */
    public function isStatic(): bool {}

    /**
     * Whether the method is magic.
     */
    public function isMagic(): bool {}

    public function getType(): string {}
}
