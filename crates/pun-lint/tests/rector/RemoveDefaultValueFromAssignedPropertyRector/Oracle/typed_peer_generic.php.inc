<?php

namespace TypedConstructorGenerics;

/** @template T */
class GenericCall
{
    private int $value = 0;

    /** @param self<int> $other */
    public function __construct(self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

/** @template T */
class GenericAssignment
{
    private int $value = 0;

    /** @param self<int> $other */
    public function __construct(self $other)
    {
        $other->value = 1;
    }
}

/** @template T */
class GenericForeign
{
    public function initialize(): void
    {
        $this->value = 1;
    }
}

final class ForeignCall
{
    private int $value = 0;

    /** @param GenericForeign<int> $other */
    public function __construct(GenericForeign $other)
    {
        $other->initialize();
    }
}
-----
<?php

namespace TypedConstructorGenerics;

/** @template T */
class GenericCall
{
    private int $value;

    /** @param self<int> $other */
    public function __construct(self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

/** @template T */
class GenericAssignment
{
    private int $value;

    /** @param self<int> $other */
    public function __construct(self $other)
    {
        $other->value = 1;
    }
}

/** @template T */
class GenericForeign
{
    public function initialize(): void
    {
        $this->value = 1;
    }
}

final class ForeignCall
{
    private int $value = 0;

    /** @param GenericForeign<int> $other */
    public function __construct(GenericForeign $other)
    {
        $other->initialize();
    }
}
