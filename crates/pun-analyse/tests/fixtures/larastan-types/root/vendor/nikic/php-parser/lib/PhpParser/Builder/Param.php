<?php declare(strict_types=1);

namespace PhpParser\Builder;

use PhpParser;
use PhpParser\BuilderHelpers;
use PhpParser\Modifiers;
use PhpParser\Node;

class Param implements PhpParser\Builder {
    protected string $name;
    protected ?Node\Expr $default = null;
    /** @var Node\Identifier|Node\Name|Node\ComplexType|null */
    protected ?Node $type = null;
    protected bool $byRef = false;
    protected int $flags = 0;
    protected bool $variadic = false;
    /** @var list<Node\AttributeGroup> */
    protected array $attributeGroups = [];

    /**
     * Creates a parameter builder.
     *
     * @param string $name Name of the parameter
     */
    public function __construct(string $name) {}

    /**
     * Sets default value for the parameter.
     *
     * @param mixed $value Default value to use
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function setDefault($value) {}

    /**
     * Sets type for the parameter.
     *
     * @param string|Node\Name|Node\Identifier|Node\ComplexType $type Parameter type
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function setType($type) {}

    /**
     * Make the parameter accept the value by reference.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeByRef() {}

    /**
     * Make the parameter variadic
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeVariadic() {}

    /**
     * Makes the (promoted) parameter public.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePublic() {}

    /**
     * Makes the (promoted) parameter protected.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeProtected() {}

    /**
     * Makes the (promoted) parameter private.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePrivate() {}

    /**
     * Makes the (promoted) parameter readonly.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeReadonly() {}

    /**
     * Gives the promoted property private(set) visibility.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePrivateSet() {}

    /**
     * Gives the promoted property protected(set) visibility.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeProtectedSet() {}

    /**
     * Adds an attribute group.
     *
     * @param Node\Attribute|Node\AttributeGroup $attribute
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function addAttribute($attribute) {}

    /**
     * Returns the built parameter node.
     *
     * @return Node\Param The built parameter node
     */
    public function getNode(): Node {}
}
