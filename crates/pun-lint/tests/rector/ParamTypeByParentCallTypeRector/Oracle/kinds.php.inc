<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

final class Other
{
}

trait Greets
{
    public function greet(?Other $other)
    {
    }
}

class Base
{
    use Greets;

    public $name = 'base';

    public function take(Other $other, Left&Right $both, (Left&Right)|null $either)
    {
    }

    public function plain($value)
    {
    }
}

final class Child extends Base
{
    private $local;

    public function greet($other)
    {
        parent::greet($other);
    }

    public function take($other, $both, $either)
    {
        parent::take(other: $other, both: $both, either: $either);
    }

    public function plain($value)
    {
        parent::plain($value);
    }

    public function missing($value)
    {
        parent::missing($value);
    }

    public function typed(int $value)
    {
        parent::typed($value);
    }
}

final class Unknown extends \Missing\ParentClass
{
    public function run($value)
    {
        parent::run($value);
    }
}

final class Failure extends \Exception
{
    public function __construct($message)
    {
        parent::__construct($message);
    }
}

function make()
{
    return new class extends Base {
        public function plain($value)
        {
            parent::plain($value);
        }

        public function take($other, $both, $either)
        {
            parent::take($other, $both, $either);
        }
    };
}
-----
<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

final class Other
{
}

trait Greets
{
    public function greet(?Other $other)
    {
    }
}

class Base
{
    use Greets;

    public $name = 'base';

    public function take(Other $other, Left&Right $both, (Left&Right)|null $either)
    {
    }

    public function plain($value)
    {
    }
}

final class Child extends Base
{
    private $local;

    public function greet(?\Probe\Other $other)
    {
        parent::greet($other);
    }

    public function take(\Probe\Other $other, \Probe\Left&\Probe\Right $both, (\Probe\Left&\Probe\Right)|null $either)
    {
        parent::take(other: $other, both: $both, either: $either);
    }

    public function plain($value)
    {
        parent::plain($value);
    }

    public function missing($value)
    {
        parent::missing($value);
    }

    public function typed(int $value)
    {
        parent::typed($value);
    }
}

final class Unknown extends \Missing\ParentClass
{
    public function run($value)
    {
        parent::run($value);
    }
}

final class Failure extends \Exception
{
    public function __construct($message)
    {
        parent::__construct($message);
    }
}

function make()
{
    return new class extends Base {
        public function plain($value)
        {
            parent::plain($value);
        }

        public function take(\Probe\Other $other, \Probe\Left&\Probe\Right $both, (\Probe\Left&\Probe\Right)|null $either)
        {
            parent::take($other, $both, $either);
        }
    };
}
