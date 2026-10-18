<?php

namespace ClassNames;

class Foo
{
}

interface Bar
{
}

trait Baz
{
}

enum Kind
{
}

class Failure extends \Exception
{
}

class Statics
{
    public static function make(): void
    {
    }
}

class Magic
{
    public static function __callStatic(string $name, array $arguments): mixed
    {
        return null;
    }
}

function relative(namespace\FOO $x): void
{
}
