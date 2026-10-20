<?php

namespace CoverageProperties;

/**
 * @property string $magic
 * @property-read int $magicRead
 */
#[\AllowDynamicProperties]
class Holder
{
    public int $i = 0;

    public static int $s = 0;

    public readonly int $r;

    /** @readonly */
    public int $doc = 0;

    /** @phpstan-readonly-allow-private-mutation */
    public int $mutable = 0;

    /** @var \ArrayAccess<int, int> */
    public readonly \ArrayAccess $storage;

    /** @var array<int, int> */
    public array $list = [];

    public function __construct(\ArrayAccess $storage)
    {
        $this->r = 1;
        $other = new self($storage);
        $other->r = 2;
        $this->storage = $storage;
    }

    public function write(): void
    {
        $this->i = 'x';
        self::$s = 'y';
        static::$s = 1;
        $this->r = 3;
        $this->doc = 4;
        $this->mutable = 5;
        $this->storage[1] = 2;
        $this->list[] = 1;
        $this->i++;
        --$this->doc;
        $this->i .= 'z';
        [$this->i, $this->doc] = [1, 2];
        $ref = &$this->r;
        $this->i = &$ref;
        unset($this->list[0]);
        $this->magic = 1;
        $this->magicRead = 1;
    }
}

/**
 * @property-read int $magicRead
 */
#[\AllowDynamicProperties]
class Other
{
}

function union(Holder|Other $u): void
{
    $u->magicRead = 3;
}

/** @immutable */
class Frozen
{
    public int $a = 0;
}

class Thawed extends Frozen
{
    public function set(): void
    {
        $this->a = 1;
    }
}

enum Suit: string
{
    case Hearts = 'h';
}

class Child extends Holder
{
    public function write(): void
    {
        parent::$s = 'z';
        $this->r = 1;
    }
}

function outside(Holder $h, Suit $s, Frozen $f): void
{
    $h->r = 1;
    $h->doc = 1;
    $h->i = 'a';
    $h->magicRead = 2;
    $s->value = 'x';
    $f->a = 2;
    Holder::$s = 'q';
}

class Hooked
{
    /** @readonly */
    public int $h {
        set {
            $this->h = $value;
        }
    }
}

class Constants
{
    const int NATIVE = 'a';

    /** @var string */
    const DOC = 1;

    /** @var true */
    const MAYBE2 = PHP_ZTS === 1;

    /** @var int<1, 5> */
    const int BOTH = 9;

    /** @var int */
    const FINE = 1;
}
