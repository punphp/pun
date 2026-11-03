<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node;

class Trait_ extends ClassLike {
    /**
     * Constructs a trait node.
     *
     * @param string|Node\Identifier $name Name
     * @param array{
     *     stmts?: Node\Stmt[],
     *     attrGroups?: Node\AttributeGroup[],
     * } $subNodes Array of the following optional subnodes:
     *             'stmts'      => array(): Statements
     *             'attrGroups' => array(): PHP attribute groups
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, array $subNodes = [], array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
