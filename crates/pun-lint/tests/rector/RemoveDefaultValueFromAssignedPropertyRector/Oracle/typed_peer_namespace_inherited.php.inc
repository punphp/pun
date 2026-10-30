<?php

namespace TypedConstructorInherited;

class ParentWriter
{
    protected function initialize(bool $stop = false): void
    {
        if ($stop) {
            return;
        }
        $this->value = 1;
    }
}

final class RelativeChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\RelativeChild::initialize();
    }
}

final class ParentNamedChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\ParentWriter::initialize();
    }
}

final class KeywordChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        parent::initialize();
    }
}

final class StaticChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        static::initialize();
    }
}

final class SelfChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        self::initialize();
    }
}

final class InheritedThisReturn extends ParentWriter
{
    protected int $value = 0;

    public function __construct(bool $stop)
    {
        $this->initialize($stop);
    }
}

final class CaseChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\casechild::INITIALIZE();
    }
}

final class OverrideChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\OverrideChild::initialize();
    }

    protected function initialize(bool $stop = false): void
    {
    }
}

class UnrelatedWriter
{
    public static function initialize(): void
    {
        self::$value = 1;
    }
}

final class UnrelatedChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\UnrelatedWriter::initialize();
    }
}
-----
<?php

namespace TypedConstructorInherited;

class ParentWriter
{
    protected function initialize(bool $stop = false): void
    {
        if ($stop) {
            return;
        }
        $this->value = 1;
    }
}

final class RelativeChild extends ParentWriter
{
    protected int $value;

    public function __construct()
    {
        namespace\RelativeChild::initialize();
    }
}

final class ParentNamedChild extends ParentWriter
{
    protected int $value;

    public function __construct()
    {
        namespace\ParentWriter::initialize();
    }
}

final class KeywordChild extends ParentWriter
{
    protected int $value;

    public function __construct()
    {
        parent::initialize();
    }
}

final class StaticChild extends ParentWriter
{
    protected int $value;

    public function __construct()
    {
        static::initialize();
    }
}

final class SelfChild extends ParentWriter
{
    protected int $value;

    public function __construct()
    {
        self::initialize();
    }
}

final class InheritedThisReturn extends ParentWriter
{
    protected int $value;

    public function __construct(bool $stop)
    {
        $this->initialize($stop);
    }
}

final class CaseChild extends ParentWriter
{
    protected int $value;

    public function __construct()
    {
        namespace\casechild::INITIALIZE();
    }
}

final class OverrideChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\OverrideChild::initialize();
    }

    protected function initialize(bool $stop = false): void
    {
    }
}

class UnrelatedWriter
{
    public static function initialize(): void
    {
        self::$value = 1;
    }
}

final class UnrelatedChild extends ParentWriter
{
    protected int $value = 0;

    public function __construct()
    {
        namespace\UnrelatedWriter::initialize();
    }
}
