<?php

namespace TypedConstructorTemplateBounds;

class TemplateCall
{
    private int $value = 0;

    /**
     * @template T of self
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

class TemplateAssignment
{
    private int $value = 0;

    /**
     * @template T of self
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}
-----
<?php

namespace TypedConstructorTemplateBounds;

class TemplateCall
{
    private int $value;

    /**
     * @template T of self
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

class TemplateAssignment
{
    private int $value;

    /**
     * @template T of self
     * @param T $other
     */
    public function __construct(object $other)
    {
        $other->value = 1;
    }
}
