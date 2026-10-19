<?php

namespace App;

interface Runs
{
    public function run();
}

abstract class Grand
{
    protected function run()
    {
    }

    public function stop()
    {
    }

    protected static function make()
    {
    }

    public function __toString(): string
    {
        return '';
    }

    abstract protected function build();
}

abstract class Middle extends Grand
{
}

class Child extends Middle
{
    function run()
    {
    }

    protected function stop()
    {
    }

    static public function make()
    {
    }

    public function __toString(): string
    {
        return 'x';
    }

    final public function build()
    {
    }

    public function own()
    {
    }
}

class Implementer extends Grand implements Runs
{
    public function run()
    {
    }
}

class Unknown extends \Vendor\Missing
{
    public function run()
    {
    }
}

abstract class Deep extends Middle
{
    abstract public function build();
}
-----
<?php

namespace App;

interface Runs
{
    public function run();
}

abstract class Grand
{
    protected function run()
    {
    }

    public function stop()
    {
    }

    protected static function make()
    {
    }

    public function __toString(): string
    {
        return '';
    }

    abstract protected function build();
}

abstract class Middle extends Grand
{
}

class Child extends Middle
{
    protected function run()
    {
    }

    public function stop()
    {
    }

    protected static function make()
    {
    }

    public function __toString(): string
    {
        return 'x';
    }

    final protected function build()
    {
    }

    public function own()
    {
    }
}

class Implementer extends Grand implements Runs
{
    public function run()
    {
    }
}

class Unknown extends \Vendor\Missing
{
    public function run()
    {
    }
}

abstract class Deep extends Middle
{
    abstract protected function build();
}
