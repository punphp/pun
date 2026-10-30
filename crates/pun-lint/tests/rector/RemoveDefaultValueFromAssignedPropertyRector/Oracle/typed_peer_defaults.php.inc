<?php

namespace ConstructorPeerBoundaries;

final class PeerCall
{
    private int $value = 0;

    public function __construct(self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class PeerEarlyReturn
{
    private int $value = 0;

    public function __construct(self $other, bool $stop)
    {
        $other->initialize($stop);
    }

    private function initialize(bool $stop): void
    {
        if ($stop) {
            return;
        }
        $this->value = 2;
    }
}

class ParentInitializer
{
    protected int $value = 0;

    protected function initialize(): void
    {
        $this->value = 3;
    }
}

final class InheritedCall extends ParentInitializer
{
    protected int $value = 0;

    public function __construct()
    {
        InheritedCall::initialize();
    }
}

final class PeerAssignment
{
    private int $value = 0;

    public function __construct(self $other)
    {
        $other->value = 4;
    }
}
-----
<?php

namespace ConstructorPeerBoundaries;

final class PeerCall
{
    private int $value;

    public function __construct(self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class PeerEarlyReturn
{
    private int $value;

    public function __construct(self $other, bool $stop)
    {
        $other->initialize($stop);
    }

    private function initialize(bool $stop): void
    {
        if ($stop) {
            return;
        }
        $this->value = 2;
    }
}

class ParentInitializer
{
    protected int $value = 0;

    protected function initialize(): void
    {
        $this->value = 3;
    }
}

final class InheritedCall extends ParentInitializer
{
    protected int $value;

    public function __construct()
    {
        InheritedCall::initialize();
    }
}

final class PeerAssignment
{
    private int $value;

    public function __construct(self $other)
    {
        $other->value = 4;
    }
}
