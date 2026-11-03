<?php declare(strict_types=1);

namespace PhpParser\Builder;

use PhpParser;
use PhpParser\BuilderHelpers;
use PhpParser\Modifiers;
use PhpParser\Node;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Stmt;
use PhpParser\Node\ComplexType;

class Property implements PhpParser\Builder {
    protected string $name;

    protected int $flags = 0;

    protected ?Node\Expr $default = null;
    /** @var array<string, mixed> */
    protected array $attributes = [];
    /** @var null|Identifier|Name|ComplexType */
    protected ?Node $type = null;
    /** @var list<Node\AttributeGroup> */
    protected array $attributeGroups = [];
    /** @var list<Node\PropertyHook> */
    protected array $hooks = [];

    /**
     * Creates a property builder.
     *
     * @param string $name Name of the property
     */
    public function __construct(string $name) {}

    /**
     * Makes the property public.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePublic() {}

    /**
     * Makes the property protected.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeProtected() {}

    /**
     * Makes the property private.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePrivate() {}

    /**
     * Makes the property static.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeStatic() {}

    /**
     * Makes the property readonly.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeReadonly() {}

    /**
     * Makes the property abstract. Requires at least one property hook to be specified as well.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeAbstract() {}

    /**
     * Makes the property final.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeFinal() {}

    /**
     * Gives the property private(set) visibility.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePrivateSet() {}

    /**
     * Gives the property protected(set) visibility.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeProtectedSet() {}

    /**
     * Sets default value for the property.
     *
     * @param mixed $value Default value to use
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function setDefault($value) {}

    /**
     * Sets doc comment for the property.
     *
     * @param PhpParser\Comment\Doc|string $docComment Doc comment to set
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function setDocComment($docComment) {}

    /**
     * Sets the property type for PHP 7.4+.
     *
     * @param string|Name|Identifier|ComplexType $type
     *
     * @return $this
     */
    public function setType($type) {}

    /**
     * Adds an attribute group.
     *
     * @param Node\Attribute|Node\AttributeGroup $attribute
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function addAttribute($attribute) {}

    /**
     * Adds a property hook.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function addHook(Node\PropertyHook $hook) {}

    /**
     * Returns the built class node.
     *
     * @return Stmt\Property The built property node
     */
    public function getNode(): PhpParser\Node {}
}
