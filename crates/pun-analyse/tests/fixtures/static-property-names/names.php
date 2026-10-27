<?php

namespace StaticPropertyNames;

class Base
{
    private static $inheritedPrivate;
    protected static $inheritedProtected;
}

class Holder extends Base
{
    public static $present;
    public $instance;
    private static $secret;
    protected static $protected;
    public static $UPPER;

    /** @param 'present'|'missing'|'other' $name */
    public function finite(string $name): void
    {
        echo self::${$name};
        self::${$name} = 1;
        self::${$name} += 1;
        self::${$name}[] = 1;
        self::${$name}++;
        echo static::${$name};
        echo self::$$name;
    }

    public function expressions(): void
    {
        echo self::${'missing'};
        self::${'missing'} = 1;
        echo self::${('missing')};
        echo self::${'miss' . 'ing'};
        $name = 'missing';
        $alias = 'name';
        echo self::$$$alias;
        echo self::${'instance'};
        echo self::${'secret'};
        echo self::${'protected'};
        echo self::${'inheritedPrivate'};
        echo parent::${'inheritedPrivate'};
        echo parent::${'inheritedProtected'};
        echo self::${'upper'};
        echo self::${'UPPER'};
        echo self::${''};
        echo self::${'$present'};
        echo self::${'0'};
    }

    /** @param 'present'|'missing' $name */
    public function guards(string $name): void
    {
        echo isset(self::${$name});
        echo self::${$name} ?? null;
        if (property_exists(self::class, 'missing')) {
            echo self::${'missing'};
        }
        if (isset(self::$checked)) {
            echo self::${'checked'};
        }
        echo self::${'unguarded'};
    }

    /**
     * @param 'missing'|int $stringOrInt
     * @param 'missing'|null $nullable
     * @param 'missing'|false $stringOrFalse
     * @param literal-string $literal
     */
    public function nonconstant(string $name, $mixed, int $int, $stringOrInt, $nullable, $stringOrFalse, string $literal, array $array, object $object, \Stringable $stringable): void
    {
        echo self::${$name};
        echo self::${$mixed};
        echo self::${$int};
        echo self::${$stringOrInt};
        echo self::${$nullable};
        echo self::${$stringOrFalse};
        echo self::${$literal};
        echo self::${$array};
        echo self::${$object};
        echo self::${$stringable};
        self::${$array} = 1;
        self::${$object} = 1;
        echo self::${42};
        echo self::${null};
    }

    /** @template T of 'present'|'missing'
     * @param T $name
     */
    public function template(string $name): void
    {
        echo self::${$name};
    }

    /** @param 'missing'|'other' $name */
    public function narrowed(string $name): void
    {
        if ($name === 'missing') {
            echo self::${$name};
        } else {
            self::${$name} = 1;
        }
    }
}

/** @param 'secret'|'protected'|'inheritedPrivate'|'inheritedProtected' $name */
function visibility(string $name): void
{
    echo Holder::${$name};
    Holder::${$name} = 1;
    echo Holder::$secret;
}

/** @param 'present'|'missing'|'instance' $name */
function receivers(Holder $holder, string $class, int $int, $mixed, string $name): void
{
    echo $holder::${$name};
    $holder::${$name} = 1;
    echo $class::${$name};
    echo $int::${$name};
    echo $mixed::${$name};
    echo Holder::$missing;
    echo $holder->instance;
}

/** @param 'present'|'missing' $name */
function unknown(string $name): void
{
    echo Missing::${$name};
    if (class_exists(Missing::class)) {
        echo Missing::${$name};
    }
    echo self::${$name};
    echo static::${$name};
    echo parent::${$name};
}

class NoParent
{
    public function read(): void
    {
        echo parent::${'missing'};
    }
}
