<?php

namespace App;

interface Greets
{
    public function greet($name);
}

abstract class Base implements Greets
{
    abstract public function shout($name);

    public function greet($name)
    {
        return sprintf('Hello %s', $name);
    }

    private function whisper($name)
    {
        return sprintf('psst %s', $name);
    }

    public function first($name, $other)
    {
        return sprintf('%s and %s', $other, $name);
    }

    public function digits($count)
    {
        return sprintf('%d items %s', $count);
    }

    public function byReference(&$name, ...$names)
    {
        return sprintf('%s', $name);
    }

    public function variadics(...$names)
    {
        return sprintf('%s', $names);
    }

    public function withDefault($name = 'x', $count = 1)
    {
        return sprintf('%s', $name) . sprintf('%s', $count);
    }

    public function defaulted($name = 'x')
    {
        return sprintf('%s', $name);
    }

    public function typed(int $name)
    {
        return sprintf('%s', $name);
    }

    public function onlyFormat($name)
    {
        return sprintf('%s');
    }

    public function noParams()
    {
        return sprintf('%s', 1);
    }

    public function assigned($name)
    {
        $name = trim($name);
        return sprintf('%s', $name);
    }

    public function ternary($name)
    {
        $x = $name ? 1 : 2;
        return sprintf('%s', $name);
    }

    public function switched($name)
    {
        switch ($name) {
            case 1:
                return 'one';
            default:
                return sprintf('%s', $name);
        }
    }

    public function nested($name)
    {
        $f = function ($inner) {
            return sprintf('%s', $inner);
        };
        return sprintf('%s', $name);
    }

    public function notVariable($name)
    {
        return sprintf('%s', $name . '!');
    }

    public function nonLiteral($name)
    {
        return sprintf(FORMAT, $name);
    }
}

function plain($name)
{
    return \sprintf('Hi %s', $name);
}

function percent($name)
{
    return sprintf('100%% %s', $name);
}

$f = fn ($name, $x) => sprintf('%s', $name);
-----
<?php

namespace App;

interface Greets
{
    public function greet($name);
}

abstract class Base implements Greets
{
    abstract public function shout($name);

    public function greet($name)
    {
        return sprintf('Hello %s', $name);
    }

    private function whisper(string $name)
    {
        return sprintf('psst %s', $name);
    }

    public function first($name, string $other)
    {
        return sprintf('%s and %s', $other, $name);
    }

    public function digits($count)
    {
        return sprintf('%d items %s', $count);
    }

    public function byReference(string &$name, ...$names)
    {
        return sprintf('%s', $name);
    }

    public function variadics(string ...$names)
    {
        return sprintf('%s', $names);
    }

    public function withDefault($name = 'x', $count = 1)
    {
        return sprintf('%s', $name) . sprintf('%s', $count);
    }

    public function defaulted(string $name = 'x')
    {
        return sprintf('%s', $name);
    }

    public function typed(int $name)
    {
        return sprintf('%s', $name);
    }

    public function onlyFormat($name)
    {
        return sprintf('%s');
    }

    public function noParams()
    {
        return sprintf('%s', 1);
    }

    public function assigned($name)
    {
        $name = trim($name);
        return sprintf('%s', $name);
    }

    public function ternary($name)
    {
        $x = $name ? 1 : 2;
        return sprintf('%s', $name);
    }

    public function switched(string $name)
    {
        switch ($name) {
            case 1:
                return 'one';
            default:
                return sprintf('%s', $name);
        }
    }

    public function nested(string $name)
    {
        $f = function (string $inner) {
            return sprintf('%s', $inner);
        };
        return sprintf('%s', $name);
    }

    public function notVariable($name)
    {
        return sprintf('%s', $name . '!');
    }

    public function nonLiteral($name)
    {
        return sprintf(FORMAT, $name);
    }
}

function plain(string $name)
{
    return \sprintf('Hi %s', $name);
}

function percent(string $name)
{
    return sprintf('100%% %s', $name);
}

$f = fn (string $name, $x) => sprintf('%s', $name);
