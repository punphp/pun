<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node;

class Enum_ extends ClassLike {
    /** @var null|Node\Identifier Scalar Type */
    public ?Node $scalarType;
    /** @var Node\Name[] Names of implemented interfaces */
    public array $implements;

    /**
     * @param string|Node\Identifier|null $name Name
     * @param array{
     *     scalarType?: Node\Identifier|null,
     *     implements?: Node\Name[],
     *     stmts?: Node\Stmt[],
     *     attrGroups?: Node\AttributeGroup[],
     * } $subNodes Array of the following optional subnodes:
     *             'scalarType'  => null    : Scalar type
     *             'implements'  => array() : Names of implemented interfaces
     *             'stmts'       => array() : Statements
     *             'attrGroups'  => array() : PHP attribute groups
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, array $subNodes = [], array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
