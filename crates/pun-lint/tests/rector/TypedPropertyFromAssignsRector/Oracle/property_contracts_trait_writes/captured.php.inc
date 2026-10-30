<?php

namespace QualityPropertyContracts;

trait PrefixWrites
{
    public function increment(): void
    {
        ++$this->prefix;
    }
}

trait OffsetWrites
{
    public function writeOffset(): void
    {
        $this->offset[0] = 1;
    }
}

trait AppendWrites
{
    public function append(): void
    {
        $this->append[] = 1;
    }
}

trait SelfWrites
{
    public function writeSelf(): void
    {
        self::$self = 1;
    }
}

trait StaticWrites
{
    public function writeStatic(): void
    {
        static::$static = 1;
    }
}

trait ReferenceWrites
{
    public function writeReference(): void
    {
        $reference = &$this->reference;
    }
}

trait UnsetWrites
{
    public function unsetValue(): void
    {
        unset($this->unset);
    }
}

final class TraitWrites
{
    use PrefixWrites, OffsetWrites, AppendWrites, SelfWrites, StaticWrites, ReferenceWrites, UnsetWrites;

    private $prefix;
    private $offset;
    private $append;
    private static $self;
    private static $static;
    private $reference;
    private $unset;
    private $untouched;

    public function __construct()
    {
        $this->prefix = 1;
        $this->offset = [];
        $this->append = [];
        self::$self = 1;
        static::$static = 1;
        $this->reference = 1;
        $this->unset = 1;
        $this->untouched = 'typed';
    }
}
-----
<?php

namespace QualityPropertyContracts;

trait PrefixWrites
{
    public function increment(): void
    {
        ++$this->prefix;
    }
}

trait OffsetWrites
{
    public function writeOffset(): void
    {
        $this->offset[0] = 1;
    }
}

trait AppendWrites
{
    public function append(): void
    {
        $this->append[] = 1;
    }
}

trait SelfWrites
{
    public function writeSelf(): void
    {
        self::$self = 1;
    }
}

trait StaticWrites
{
    public function writeStatic(): void
    {
        static::$static = 1;
    }
}

trait ReferenceWrites
{
    public function writeReference(): void
    {
        $reference = &$this->reference;
    }
}

trait UnsetWrites
{
    public function unsetValue(): void
    {
        unset($this->unset);
    }
}

final class TraitWrites
{
    use PrefixWrites, OffsetWrites, AppendWrites, SelfWrites, StaticWrites, ReferenceWrites, UnsetWrites;

    private $prefix;
    private $offset;
    private $append;
    private static $self;
    private static $static;
    private $reference;
    private $unset;
    private string $untouched;

    public function __construct()
    {
        $this->prefix = 1;
        $this->offset = [];
        $this->append = [];
        self::$self = 1;
        static::$static = 1;
        $this->reference = 1;
        $this->unset = 1;
        $this->untouched = 'typed';
    }
}
