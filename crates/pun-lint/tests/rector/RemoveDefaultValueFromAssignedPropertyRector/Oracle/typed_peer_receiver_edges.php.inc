<?php

final class TypedPeerThisAlias
{
    private int $value = 0;

    public function __construct()
    {
        $other = $this;
        $other->value = 1;
    }
}

class TypedPeerGlobalParent
{
    protected function initialize(): void
    {
        $this->value = 1;
    }
}

final class TypedPeerGlobalChild extends TypedPeerGlobalParent
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\TypedPeerGlobalChild::initialize();
    }
}

final class TypedPeerObjectStatic
{
    private int $value = 0;

    public function __construct(self $other)
    {
        $other::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class TypedPeerUnknownStatic
{
    private int $value = 0;

    public function __construct(string $class)
    {
        $class::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class TypedPeerBuiltin extends \Exception
{
    private int $value = 0;

    public function __construct()
    {
        $this->getMessage();
    }
}
-----
<?php

final class TypedPeerThisAlias
{
    private int $value;

    public function __construct()
    {
        $other = $this;
        $other->value = 1;
    }
}

class TypedPeerGlobalParent
{
    protected function initialize(): void
    {
        $this->value = 1;
    }
}

final class TypedPeerGlobalChild extends TypedPeerGlobalParent
{
    protected int $value;

    public function __construct()
    {
        namespace\TypedPeerGlobalChild::initialize();
    }
}

final class TypedPeerObjectStatic
{
    private int $value;

    public function __construct(self $other)
    {
        $other::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class TypedPeerUnknownStatic
{
    private int $value = 0;

    public function __construct(string $class)
    {
        $class::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class TypedPeerBuiltin extends \Exception
{
    private int $value = 0;

    public function __construct()
    {
        $this->getMessage();
    }
}
