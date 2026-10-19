<?php

namespace App;

function stop(): never
{
    exit(1);
}

function halt()
{
    stop();
}

function bail()
{
    \App\stop();
}

function phpdocOnly()
{
    /** @var int $x */
    abortish();
}

/** @return never */
function abortish()
{
    if (true) {
        throw new \RuntimeException();
    }
}

function voidFunction(): void
{
    throw new \RuntimeException();
}

function guarded()
{
    try {
        throw new \RuntimeException();
    } finally {
        echo 1;
    }
}

function conditional($a)
{
    $a ? throw new \LogicException() : null;
    throw new \RuntimeException();
}

function looping()
{
    for (;;) {
        throw new \RuntimeException();
    }
}

final class Service
{
    public function fail(): never
    {
        throw new \RuntimeException();
    }

    public function viaThis()
    {
        $this->fail();
    }

    public function viaStatic()
    {
        static::fail();
    }

    public function viaSelf(): void
    {
        self::fail();
    }

    private function viaClosure()
    {
        $f = function (): void {
            throw new \RuntimeException();
        };
        $g = function () use ($f) {
            exit;
        };
        $h = static fn () => throw new \RuntimeException();
    }

    public function __toString()
    {
        throw new \RuntimeException();
    }

    public function __invoke()
    {
        die('x');
    }
}

class Base
{
    public function run()
    {
        throw new \RuntimeException();
    }

    protected function typed(): int
    {
        return 1;
    }
}

class Child extends Base
{
    public function run()
    {
        throw new \LogicException();
    }

    protected function typed()
    {
        throw new \LogicException();
    }
}

class Unknown extends \Vendor\Thing
{
    public function run()
    {
        throw new \LogicException();
    }

    private function hidden()
    {
        throw new \LogicException();
    }
}

class Internal extends \ArrayObject
{
    public function offsetGet($key)
    {
        throw new \LogicException();
    }

    public function own()
    {
        throw new \LogicException();
    }
}

interface Contract
{
    public function make();
}

abstract class AbstractThing implements Contract
{
    public function make()
    {
        throw new \LogicException();
    }
}

trait Helper
{
    public function help()
    {
        throw new \LogicException();
    }
}

enum Kind
{
    case A;

    public function fail()
    {
        throw new \LogicException();
    }
}
-----
<?php

namespace App;

function stop(): never
{
    exit(1);
}

function halt()
{
    stop();
}

function bail()
{
    \App\stop();
}

function phpdocOnly()
{
    /** @var int $x */
    abortish();
}

/** @return never */
function abortish()
{
    if (true) {
        throw new \RuntimeException();
    }
}

function voidFunction(): void
{
    throw new \RuntimeException();
}

function guarded()
{
    try {
        throw new \RuntimeException();
    } finally {
        echo 1;
    }
}

function conditional($a)
{
    $a ? throw new \LogicException() : null;
    throw new \RuntimeException();
}

function looping()
{
    for (;;) {
        throw new \RuntimeException();
    }
}

final class Service
{
    public function fail(): never
    {
        throw new \RuntimeException();
    }

    public function viaThis()
    {
        $this->fail();
    }

    public function viaStatic()
    {
        static::fail();
    }

    public function viaSelf(): void
    {
        self::fail();
    }

    private function viaClosure()
    {
        $f = function (): never {
            throw new \RuntimeException();
        };
        $g = function () use ($f): never {
            exit;
        };
        $h = static fn () => throw new \RuntimeException();
    }

    public function __toString()
    {
        throw new \RuntimeException();
    }

    public function __invoke()
    {
        die('x');
    }
}

class Base
{
    public function run()
    {
        throw new \RuntimeException();
    }

    protected function typed(): int
    {
        return 1;
    }
}

class Child extends Base
{
    public function run()
    {
        throw new \LogicException();
    }

    protected function typed()
    {
        throw new \LogicException();
    }
}

class Unknown extends \Vendor\Thing
{
    public function run()
    {
        throw new \LogicException();
    }

    private function hidden()
    {
        throw new \LogicException();
    }
}

class Internal extends \ArrayObject
{
    public function offsetGet($key)
    {
        throw new \LogicException();
    }

    public function own()
    {
        throw new \LogicException();
    }
}

interface Contract
{
    public function make();
}

abstract class AbstractThing implements Contract
{
    public function make()
    {
        throw new \LogicException();
    }
}

trait Helper
{
    public function help()
    {
        throw new \LogicException();
    }
}

enum Kind
{
    case A;

    public function fail()
    {
        throw new \LogicException();
    }
}
