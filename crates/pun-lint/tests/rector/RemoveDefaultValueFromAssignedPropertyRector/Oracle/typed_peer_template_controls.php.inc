<?php

namespace TypedConstructorTemplateControls;

class NestedAssignment
{
    private int $value = 0;

    /**
     * @template T of self
     * @template U of T
     * @param U $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}

class AmbiguousAssignment
{
    private int $value = 0;

    /**
     * @template T of self&\Countable
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}

class ForeignAssignment
{
    private int $value = 0;

    /**
     * @template T of \stdClass
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}

class UnionAssignment
{
    private int $value = 0;

    /**
     * @template T of self|\stdClass
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}
-----
<?php

namespace TypedConstructorTemplateControls;

class NestedAssignment
{
    private int $value;

    /**
     * @template T of self
     * @template U of T
     * @param U $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}

class AmbiguousAssignment
{
    private int $value = 0;

    /**
     * @template T of self&\Countable
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}

class ForeignAssignment
{
    private int $value = 0;

    /**
     * @template T of \stdClass
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}

class UnionAssignment
{
    private int $value = 0;

    /**
     * @template T of self|\stdClass
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}
