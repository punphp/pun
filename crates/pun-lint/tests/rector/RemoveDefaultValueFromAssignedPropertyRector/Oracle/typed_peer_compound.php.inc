<?php

namespace ConstructorCompoundPeers;

class AmbiguousIntersection
{
    private int $value = 0;

    public function __construct(self&\Countable $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

class RedundantIntersection implements \Countable
{
    private int $value = 0;

    public function __construct(self&\Countable $other)
    {
        $other->initialize();
    }

    public function count(): int
    {
        return 0;
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

class IntersectionAssignment
{
    private int $value = 0;

    public function __construct(self&\Countable $other)
    {
        $other->value = 1;
    }
}

final class ParentTypedPeer
{
    private int $value = 0;

    public function __construct(\stdClass $other)
    {
        $other->value = 1;
    }
}
-----
<?php

namespace ConstructorCompoundPeers;

class AmbiguousIntersection
{
    private int $value = 0;

    public function __construct(self&\Countable $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

class RedundantIntersection implements \Countable
{
    private int $value;

    public function __construct(self&\Countable $other)
    {
        $other->initialize();
    }

    public function count(): int
    {
        return 0;
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

class IntersectionAssignment
{
    private int $value = 0;

    public function __construct(self&\Countable $other)
    {
        $other->value = 1;
    }
}

final class ParentTypedPeer
{
    private int $value = 0;

    public function __construct(\stdClass $other)
    {
        $other->value = 1;
    }
}
