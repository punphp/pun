<?php

namespace ConstructorNamedContracts;

use ConstructorNamedContracts\AliasCall as Initializer;

final class NamedCall
{
    private int $value = 0;

    public function __construct()
    {
        NamedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class AliasCall
{
    private int $value = 0;

    public function __construct()
    {
        Initializer::initialize();
    }

    private function initialize(): void
    {
        $this->value = 2;
    }
}

final class FullyQualifiedCall
{
    private int $value = 0;

    public function __construct()
    {
        \ConstructorNamedContracts\FullyQualifiedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 3;
    }
}

final class NamedEarlyReturn
{
    private int $value = 0;

    public function __construct(bool $stop)
    {
        NamedEarlyReturn::initialize($stop);
    }

    private function initialize(bool $stop): void
    {
        if ($stop) {
            return;
        }
        $this->value = 4;
    }
}

final class ThisEarlyReturn
{
    private int $value = 0;

    public function __construct(bool $stop)
    {
        $this->initialize($stop);
    }

    private function initialize(bool $stop): void
    {
        if ($stop) {
            return;
        }
        $this->value = 5;
    }
}

final class ConstructorEarlyReturn
{
    private int $value = 0;

    public function __construct(bool $stop)
    {
        if ($stop) {
            return;
        }
        ConstructorEarlyReturn::initialize();
    }

    private function initialize(): void
    {
        $this->value = 6;
    }
}

final class AssignmentTarget
{
    private int $value = 0;

    public function __construct()
    {
        $unused = AssignmentTarget::initialize();
    }

    private function initialize(): void
    {
        $this->value = 7;
    }
}

final class TransitiveCall
{
    private int $value = 0;

    public function __construct()
    {
        TransitiveCall::outer();
    }

    private function outer(): void
    {
        $this->initialize();
    }

    private function initialize(): void
    {
        $this->value = 8;
    }
}

final class ForeignCall
{
    private int $value = 0;

    public function __construct()
    {
        ForeignInitializer::initialize();
    }

    private function initialize(): void
    {
        $this->value = 9;
    }
}

final class ForeignPeer
{
    private int $value = 0;

    public function __construct(ForeignInitializer $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 10;
    }
}

final class ForeignInitializer
{
    public static function initialize(): void
    {
    }
}
-----
<?php

namespace ConstructorNamedContracts;

use ConstructorNamedContracts\AliasCall as Initializer;

final class NamedCall
{
    private int $value;

    public function __construct()
    {
        NamedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class AliasCall
{
    private int $value;

    public function __construct()
    {
        Initializer::initialize();
    }

    private function initialize(): void
    {
        $this->value = 2;
    }
}

final class FullyQualifiedCall
{
    private int $value;

    public function __construct()
    {
        \ConstructorNamedContracts\FullyQualifiedCall::initialize();
    }

    private function initialize(): void
    {
        $this->value = 3;
    }
}

final class NamedEarlyReturn
{
    private int $value;

    public function __construct(bool $stop)
    {
        NamedEarlyReturn::initialize($stop);
    }

    private function initialize(bool $stop): void
    {
        if ($stop) {
            return;
        }
        $this->value = 4;
    }
}

final class ThisEarlyReturn
{
    private int $value = 0;

    public function __construct(bool $stop)
    {
        $this->initialize($stop);
    }

    private function initialize(bool $stop): void
    {
        if ($stop) {
            return;
        }
        $this->value = 5;
    }
}

final class ConstructorEarlyReturn
{
    private int $value = 0;

    public function __construct(bool $stop)
    {
        if ($stop) {
            return;
        }
        ConstructorEarlyReturn::initialize();
    }

    private function initialize(): void
    {
        $this->value = 6;
    }
}

final class AssignmentTarget
{
    private int $value = 0;

    public function __construct()
    {
        $unused = AssignmentTarget::initialize();
    }

    private function initialize(): void
    {
        $this->value = 7;
    }
}

final class TransitiveCall
{
    private int $value = 0;

    public function __construct()
    {
        TransitiveCall::outer();
    }

    private function outer(): void
    {
        $this->initialize();
    }

    private function initialize(): void
    {
        $this->value = 8;
    }
}

final class ForeignCall
{
    private int $value = 0;

    public function __construct()
    {
        ForeignInitializer::initialize();
    }

    private function initialize(): void
    {
        $this->value = 9;
    }
}

final class ForeignPeer
{
    private int $value = 0;

    public function __construct(ForeignInitializer $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 10;
    }
}

final class ForeignInitializer
{
    public static function initialize(): void
    {
    }
}
