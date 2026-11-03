<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\Modifiers;
use PhpParser\Node\Expr\Assign;
use PhpParser\Node\Expr\PropertyFetch;
use PhpParser\Node\Expr\Variable;
use PhpParser\Node\Stmt\Expression;
use PhpParser\Node\Stmt\Return_;
use PhpParser\NodeAbstract;

class PropertyHook extends NodeAbstract implements FunctionLike {
    /** @var AttributeGroup[] PHP attribute groups */
    public array $attrGroups;
    /** @var int Modifiers */
    public int $flags;
    /** @var bool Whether hook returns by reference */
    public bool $byRef;
    /** @var Identifier Hook name */
    public Identifier $name;
    /** @var Param[] Parameters */
    public array $params;
    /** @var null|Expr|Stmt[] Hook body */
    public $body;

    /**
     * Constructs a property hook node.
     *
     * @param string|Identifier $name Hook name
     * @param null|Expr|Stmt[] $body Hook body
     * @param array{
     *     flags?: int,
     *     byRef?: bool,
     *     params?: Param[],
     *     attrGroups?: AttributeGroup[],
     * } $subNodes Array of the following optional subnodes:
     *             'flags       => 0      : Flags
     *             'byRef'      => false  : Whether hook returns by reference
     *             'params'     => array(): Parameters
     *             'attrGroups' => array(): PHP attribute groups
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, $body, array $subNodes = [], array $attributes = []) {}

    public function returnsByRef(): bool {}

    public function getParams(): array {}

    public function getReturnType() {}

    /**
     * Whether the property hook is final.
     */
    public function isFinal(): bool {}

    public function getStmts(): ?array {}

    public function getAttrGroups(): array {}

    public function getType(): string {}

    public function getSubNodeNames(): array {}
}
