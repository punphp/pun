<?php

namespace ConstructorDocumentedPeers;

class ForeignInitializer
{
    public function initialize(): void
    {
        $this->value = 1;
    }
}

final class DocumentedPeer
{
    private int $value = 0;

    /** @param self $other */
    public function __construct(object $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class DocumentedAssignment
{
    private int $value = 0;

    /** @param self $other */
    public function __construct(object $other)
    {
        $other->value = 1;
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
        $this->value = 1;
    }
}

final class NullableCall
{
    private int $value = 0;

    public function __construct(?self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class NullableAssignment
{
    private int $value = 0;

    public function __construct(?self $other)
    {
        $other->value = 1;
    }
}

final class NoTransitivePeer
{
    private int $value = 0;

    public function __construct(self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->write();
    }

    private function write(): void
    {
        $this->value = 1;
    }
}

final class NestedAssignment
{
    private int $value = 0;

    public function __construct(self $other, bool $write)
    {
        $other->initialize($write);
    }

    private function initialize(bool $write): void
    {
        if ($write) {
            $this->value = 1;
        }
    }
}

final class DirectEarlyReturn
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
        $this->value = 1;
    }
}
-----
<?php

namespace ConstructorDocumentedPeers;

class ForeignInitializer
{
    public function initialize(): void
    {
        $this->value = 1;
    }
}

final class DocumentedPeer
{
    private int $value;

    /** @param self $other */
    public function __construct(object $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class DocumentedAssignment
{
    private int $value;

    /** @param self $other */
    public function __construct(object $other)
    {
        $other->value = 1;
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
        $this->value = 1;
    }
}

final class NullableCall
{
    private int $value = 0;

    public function __construct(?self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->value = 1;
    }
}

final class NullableAssignment
{
    private int $value = 0;

    public function __construct(?self $other)
    {
        $other->value = 1;
    }
}

final class NoTransitivePeer
{
    private int $value = 0;

    public function __construct(self $other)
    {
        $other->initialize();
    }

    private function initialize(): void
    {
        $this->write();
    }

    private function write(): void
    {
        $this->value = 1;
    }
}

final class NestedAssignment
{
    private int $value = 0;

    public function __construct(self $other, bool $write)
    {
        $other->initialize($write);
    }

    private function initialize(bool $write): void
    {
        if ($write) {
            $this->value = 1;
        }
    }
}

final class DirectEarlyReturn
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
        $this->value = 1;
    }
}
