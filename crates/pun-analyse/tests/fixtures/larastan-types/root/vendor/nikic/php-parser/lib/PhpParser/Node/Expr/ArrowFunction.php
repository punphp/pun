<?php declare(strict_types=1);

namespace PhpParser\Node\Expr;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\FunctionLike;

class ArrowFunction extends Expr implements FunctionLike {
    /** @var bool Whether the closure is static */
    public bool $static;

    /** @var bool Whether to return by reference */
    public bool $byRef;

    /** @var Node\Param[] */
    public array $params = [];

    /** @var null|Node\Identifier|Node\Name|Node\ComplexType */
    public ?Node $returnType;

    /** @var Expr Expression body */
    public Expr $expr;
    /** @var Node\AttributeGroup[] */
    public array $attrGroups;

    /**
     * @param array{
     *     expr: Expr,
     *     static?: bool,
     *     byRef?: bool,
     *     params?: Node\Param[],
     *     returnType?: null|Node\Identifier|Node\Name|Node\ComplexType,
     *     attrGroups?: Node\AttributeGroup[]
     * } $subNodes Array of the following subnodes:
     *             'expr'                  : Expression body
     *             'static'     => false   : Whether the closure is static
     *             'byRef'      => false   : Whether to return by reference
     *             'params'     => array() : Parameters
     *             'returnType' => null    : Return type
     *             'attrGroups' => array() : PHP attribute groups
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $subNodes, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function returnsByRef(): bool {}

    public function getParams(): array {}

    public function getReturnType() {}

    public function getAttrGroups(): array {}

    /**
     * @return Node\Stmt\Return_[]
     */
    public function getStmts(): array {}

    public function getType(): string {}
}
