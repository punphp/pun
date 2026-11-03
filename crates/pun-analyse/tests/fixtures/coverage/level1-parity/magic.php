<?php

namespace Level1Parity;

// From level 1 (reportMagicMethods/Properties), __call and __get no longer answer every name on $this.
class Magic
{
    public function __call($method, $parameters)
    {
        return null;
    }

    public static function __callStatic($method, $parameters)
    {
        return null;
    }

    public function __get($key)
    {
        return null;
    }

    public function run(): void
    {
        $this->missing();
        echo $this->absent;
        static::missingStatic();
        self::missingStatic();
    }
}

trait UsesOwner
{
    public function read(): void
    {
        echo $this->email;
        $this->notify();
    }
}

class Owner
{
    use UsesOwner;

    public function __get($key)
    {
        return null;
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 */
abstract class Paginator
{
    public function hasPages(): bool
    {
        return $this->hasMorePages();
    }
}

class Target
{
    public int $count = 0;

    public function forwarded(): void {}

    public static function staticForwarded(): void {}

    /** @param-closure-this static $macro */
    public static function macro(string $name, \Closure $macro): void {}
}

/** @mixin Target */
class Forwarding
{
    public function __get($key)
    {
        return null;
    }

    public function __call($method, $parameters)
    {
        return null;
    }

    public function run(): void
    {
        $this->forwarded();
        echo $this->count;
        static::staticForwarded();
        $this->macro('m', function () {
            $this->notOnTarget();
        });
    }
}

interface Partial
{
    public function isOneOfMany(): bool;
}

class Relation
{
    public function __call($method, $parameters)
    {
        return null;
    }

    public function is(): bool
    {
        return $this instanceof Partial && $this->isOneOfMany();
    }
}
