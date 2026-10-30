<?php

namespace ConstructorNestedScope;

final class OuterNamedInAnonymous
{
    private int $value = 0;

    public function __construct()
    {
        $this->value = 1;
    }

    public function nested(): object
    {
        return new class
        {
            public function write(): void
            {
                OuterNamedInAnonymous::$value[] = 2;
            }
        };
    }
}

final class ConstructorNestedBranches
{
    private int $value = 0;

    public function __construct()
    {
        $nested = new class
        {
            public function write(bool $flag): void
            {
                if ($flag) {
                    ConstructorNestedBranches::$value = 3;
                } else {
                    ConstructorNestedBranches::$value = 4;
                }
            }
        };
    }
}
-----
<?php

namespace ConstructorNestedScope;

final class OuterNamedInAnonymous
{
    private int $value;

    public function __construct()
    {
        $this->value = 1;
    }

    public function nested(): object
    {
        return new class
        {
            public function write(): void
            {
                OuterNamedInAnonymous::$value[] = 2;
            }
        };
    }
}

final class ConstructorNestedBranches
{
    private int $value = 0;

    public function __construct()
    {
        $nested = new class
        {
            public function write(bool $flag): void
            {
                if ($flag) {
                    ConstructorNestedBranches::$value = 3;
                } else {
                    ConstructorNestedBranches::$value = 4;
                }
            }
        };
    }
}
