<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node;
use PhpParser\Node\PropertyItem;

abstract class ClassLike extends Node\Stmt {
    /** @var Node\Identifier|null Name */
    public ?Node\Identifier $name;
    /** @var Node\Stmt[] Statements */
    public array $stmts;
    /** @var Node\AttributeGroup[] PHP attribute groups */
    public array $attrGroups;

    /** @var Node\Name|null Namespaced name (if using NameResolver) */
    public ?Node\Name $namespacedName;

    /**
     * @return list<TraitUse>
     */
    public function getTraitUses(): array {}

    /**
     * @return list<ClassConst>
     */
    public function getConstants(): array {}

    /**
     * @return list<Property>
     */
    public function getProperties(): array {}

    /**
     * Gets property with the given name defined directly in this class/interface/trait.
     *
     * @param string $name Name of the property
     *
     * @return Property|null Property node or null if the property does not exist
     */
    public function getProperty(string $name): ?Property {}

    /**
     * Gets all methods defined directly in this class/interface/trait
     *
     * @return list<ClassMethod>
     */
    public function getMethods(): array {}

    /**
     * Gets method with the given name defined directly in this class/interface/trait.
     *
     * @param string $name Name of the method (compared case-insensitively)
     *
     * @return ClassMethod|null Method node or null if the method does not exist
     */
    public function getMethod(string $name): ?ClassMethod {}
}
