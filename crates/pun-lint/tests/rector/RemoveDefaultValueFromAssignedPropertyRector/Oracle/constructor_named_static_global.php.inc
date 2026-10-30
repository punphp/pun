<?php

final class ConstructorGlobalNamedCall
{
    private int $value = 0;

    public function __construct()
    {
        namespace\ConstructorGlobalNamedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

$anonymous = new class
{
    private int $value = 0;

    public function __construct()
    {
        ConstructorGlobalNamedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 2;
    }
};
-----
<?php

final class ConstructorGlobalNamedCall
{
    private int $value;

    public function __construct()
    {
        namespace\ConstructorGlobalNamedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

$anonymous = new class
{
    private int $value = 0;

    public function __construct()
    {
        ConstructorGlobalNamedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 2;
    }
};
