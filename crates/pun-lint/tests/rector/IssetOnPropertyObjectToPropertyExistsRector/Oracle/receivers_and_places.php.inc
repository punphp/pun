<?php

namespace Oracle\PropertyExists;

use ArrayObject;
use DateTime;

class ParentClass
{
    private $hidden;

    protected $shared;

    protected ?int $typedNull = null;
}

/**
 * @property string|null $magic
 */
class Child extends ParentClass
{
    public mixed $any;

    public ?int $noDefault;

    public function __construct(public ?int $promoted = null)
    {
    }

    public function run(array $items, ArrayObject $object, DateTime $date, ?Child $maybe, Child|ParentClass $either, Getter $getter, Status $status, Contract $contract)
    {
        isset($this->hidden);
        isset($this->shared);
        isset($this->typedNull);
        isset($this->magic);
        isset($this->any);
        isset($this->noDefault);
        isset($this->promoted);
        isset($object->flags);
        isset($date->foo);
        isset($maybe->shared);
        isset($either->shared);
        isset($getter->value);
        isset($status->name);
        isset($status->value);
        isset($contract->value);
        isset($this->shared, $items['x']);
        isset($items['x'], $this->shared);
        isset($this::$static);
        isset($maybe?->shared);
        isset($this->shared->deeper);
    }

    public function contexts($z)
    {
        $a = isset($this->shared) === false;
        $b = (bool) isset($this->shared, $this->any);
        $c = $z || ! isset($this->undeclared);
        $d = ! isset($this->shared, $this->any) || $z;
        $e = isset($this->undeclared) ? 1 : 2;
        $f = $z && isset($this->shared, $this->any);
        $g = ! isset($this->undeclared) && $z;
        $h = isset($this->undeclared) xor $z;
        $i = $z and isset($this->undeclared);
        $j = [isset($this->undeclared), ! isset($this->shared)];
        return fn () => isset($this->undeclared);
    }
}

class Getter
{
    public function __get($name)
    {
        return 1;
    }
}

enum Status: string
{
    case On = 'on';
}

interface Contract
{
}

trait UsesThis
{
    private $traitProp;

    public function check()
    {
        return isset($this->traitProp);
    }
}
-----
<?php

namespace Oracle\PropertyExists;

use ArrayObject;
use DateTime;

class ParentClass
{
    private $hidden;

    protected $shared;

    protected ?int $typedNull = null;
}

/**
 * @property string|null $magic
 */
class Child extends ParentClass
{
    public mixed $any;

    public ?int $noDefault;

    public function __construct(public ?int $promoted = null)
    {
    }

    public function run(array $items, ArrayObject $object, DateTime $date, ?Child $maybe, Child|ParentClass $either, Getter $getter, Status $status, Contract $contract)
    {
        property_exists($this, 'hidden') && $this->hidden !== null;
        $this->shared !== null;
        $this->typedNull !== null;
        property_exists($this, 'magic') && $this->magic !== null;
        $this->any !== null;
        isset($this->noDefault);
        $this->promoted !== null;
        property_exists($object, 'flags') && $object->flags !== null;
        property_exists($date, 'foo') && $date->foo !== null;
        $maybe->shared !== null;
        $either->shared !== null;
        property_exists($getter, 'value') && $getter->value !== null;
        $status->name !== null;
        $status->value !== null;
        property_exists($contract, 'value') && $contract->value !== null;
        $this->shared !== null;
        $this->shared !== null;
        isset($this::$static);
        isset($maybe?->shared);
        isset($this->shared->deeper);
    }

    public function contexts($z)
    {
        $a = ($this->shared !== null) === false;
        $b = (bool) ($this->shared !== null && $this->any !== null);
        $c = $z || (!property_exists($this, 'undeclared') || $this->undeclared === null);
        $d = $this->shared === null && $this->any === null || $z;
        $e = property_exists($this, 'undeclared') && $this->undeclared !== null ? 1 : 2;
        $f = $z && ($this->shared !== null && $this->any !== null);
        $g = (!property_exists($this, 'undeclared') || $this->undeclared === null) && $z;
        $h = property_exists($this, 'undeclared') && $this->undeclared !== null xor $z;
        $i = $z and property_exists($this, 'undeclared') && $this->undeclared !== null;
        $j = [property_exists($this, 'undeclared') && $this->undeclared !== null, $this->shared === null];
        return fn () => property_exists($this, 'undeclared') && $this->undeclared !== null;
    }
}

class Getter
{
    public function __get($name)
    {
        return 1;
    }
}

enum Status: string
{
    case On = 'on';
}

interface Contract
{
}

trait UsesThis
{
    private $traitProp;

    public function check()
    {
        return isset($this->traitProp);
    }
}
