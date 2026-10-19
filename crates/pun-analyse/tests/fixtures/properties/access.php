<?php

namespace Props;

class ParentClass
{
    private $secret;

    protected static $counter;
}

/**
 * @template TKey of int|string
 * @template TValue
 */
class Child extends ParentClass
{
    public $known;

    public function reads(): void
    {
        echo $this->known;
        echo $this->missing;
        echo $this->secret;
        echo $this->counter;
        echo $this->missing ?? 'default';
        echo isset($this->other) ? $this->other : null;
        if ($this->flag) {
            echo $this->flag;
        } else {
            echo $this->flag;
        }
        if (!isset($this->early)) {
            return;
        }
        echo $this->early;
    }

    public function writes(): void
    {
        $this->created = 1;
        echo $this->created;
        $this->items[] = 1;
        echo $this->items;
        $this->total += 1;
        $this->secret = 2;
        [$this->first, $this->second] = [1, 2];
        unset($this->gone);
        if (rand()) {
            $this->branch = 1;
        }
        echo $this->branch;
    }

    public static function statics(): void
    {
        echo $this->nothing;
    }

    public function closures(): void
    {
        $f = function () {
            return $this->inClosure;
        };
        $g = static function () {
            return $this->inStatic;
        };
        $h = fn () => $this->inArrow;
    }
}

final class Sealed
{
    public function read(): void
    {
        echo isset($this->absent) ? 1 : 0;
        echo $this->absent ?? 1;
    }
}

class Magic
{
    public function __get($name)
    {
        return null;
    }

    public function read(): void
    {
        echo $this->anything;
    }
}

/**
 * @property int $tagged
 */
#[\AllowDynamicProperties]
class Dynamic
{
    public function read(): void
    {
        echo $this->tagged;
        echo $this->untagged;
    }
}

/**
 * @property int $tagged
 */
class Tagged
{
    public function read(): void
    {
        echo $this->tagged;
    }
}

trait UsesProps
{
    public function fromTrait(): void
    {
        echo $this->viaTrait;
    }
}

class WithTrait
{
    use UsesProps;
}

function outside(): void
{
    echo $this->outside;
}

class Statics extends ParentClass
{
    public static $shared;

    private static $own;

    public $instance;

    public function read(): void
    {
        echo self::$shared;
        echo static::$missingStatic;
        echo self::$missingSelf;
        echo self::$instance;
        echo parent::$counter;
        echo Statics::$own;
        echo Unknown::$nothing;
        echo statics::$shared;
        echo isset(self::$maybe) ? 1 : 0;
        self::$assigned = 1;
        echo self::$assigned;
    }
}

class Hidden
{
    private static $secret;

    protected static $guarded;
}

class Outsider
{
    public function read(): void
    {
        echo Hidden::$secret;
        echo Hidden::$guarded;
        echo parent::$none;
    }
}

final class FinalStatics
{
    public function read(): void
    {
        echo isset(self::$absent) ? 1 : 0;
    }
}

echo self::$outside;
echo parent::$outside;

class Narrowing
{
    public function loop(): void
    {
        while (isset($this->looped)) {
            echo $this->looped;
        }
        echo $this->looped;
        if (property_exists($this, 'probed')) {
            echo $this->probed;
        }
        if (!property_exists($this, 'negated')) {
            echo $this->negated;
        }
    }

    public static function statics(): void
    {
        if (isset(static::$checked)) {
            echo static::$checked;
        }
        if (property_exists(self::class, 'exists') && null !== self::$exists) {
            echo self::$exists;
        }
        if (\property_exists(Narrowing::class, 'named')) {
            echo Narrowing::$named;
            echo self::$named;
        }
        \stdClass::$crate = 1;
    }
}

class HookedStatic
{
    #[Deprecated]
    public static string $value {
        get => $this->value;
    }
}

class Rebinding
{
    public function bind(object $o): void
    {
        (function () { echo $this->boundTo; })->bindTo($o, $o)();
        (function () { echo $this->called; })->call($o);
        \Closure::bind(function () { echo $this->bound; }, $o, $o)();
    }

    public function arguments(): object
    {
        return new class ($this->outer) {
            public function __construct(public mixed $value)
            {
            }
        };
    }
}
