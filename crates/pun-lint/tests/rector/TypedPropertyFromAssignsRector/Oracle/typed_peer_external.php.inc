<?php

namespace TypedConstructorExternal;

/** @extends Initializer<int> */
final class ExternalChild extends Initializer
{
    private $value;

    public function __construct()
    {
        $this->initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}

/** @extends Initializer<int> */
final class ExternalNamedChild extends Initializer
{
    private $value;

    public function __construct()
    {
        namespace\ExternalNamedChild::initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}

/** @extends Initializer<int> */
final class ExternalPeerChild extends Initializer
{
    private $value;

    /** @param Initializer<int> $other */
    public function __construct(Initializer $other)
    {
        $other->initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}

final class ExternalForeignControl
{
    private $value;

    public function __construct(ForeignInitializer $other)
    {
        $other->initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}
-----
<?php

namespace TypedConstructorExternal;

/** @extends Initializer<int> */
final class ExternalChild extends Initializer
{
    private int $value;

    public function __construct()
    {
        $this->initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}

/** @extends Initializer<int> */
final class ExternalNamedChild extends Initializer
{
    private int $value;

    public function __construct()
    {
        namespace\ExternalNamedChild::initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}

/** @extends Initializer<int> */
final class ExternalPeerChild extends Initializer
{
    private int $value;

    /** @param Initializer<int> $other */
    public function __construct(Initializer $other)
    {
        $other->initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}

final class ExternalForeignControl
{
    private ?int $value = null;

    public function __construct(ForeignInitializer $other)
    {
        $other->initialize();
    }

    private function write(): void
    {
        $this->value = 2;
    }
}
