<?php

namespace Pun\Oracle\UnusedPrivate;

use Codeception\Attribute\DataProvider;

final class Callers
{
    public function run($name)
    {
        register_shutdown_function([$this, 'shutdown']);
        $f = function () {
            return [static::class, 'fromClosure'];
        };
        static::fromStatic();
        $this?->fromNullsafe();
        $other = new Other();
        $other->fromOther();
        return $f;
    }

    #[DataProvider('provider')]
    public function testIt()
    {
    }

    private function shutdown()
    {
    }

    private function fromClosure()
    {
    }

    private static function fromStatic()
    {
    }

    private function fromNullsafe()
    {
    }

    private function fromOther()
    {
    }

    private function provider()
    {
    }
}

final class Other
{
    public function fromOther()
    {
    }
}
-----
<?php

namespace Pun\Oracle\UnusedPrivate;

use Codeception\Attribute\DataProvider;

final class Callers
{
    public function run($name)
    {
        register_shutdown_function([$this, 'shutdown']);
        $f = function () {
            return [static::class, 'fromClosure'];
        };
        static::fromStatic();
        $this?->fromNullsafe();
        $other = new Other();
        $other->fromOther();
        return $f;
    }

    #[DataProvider('provider')]
    public function testIt()
    {
    }

    private function fromClosure()
    {
    }

    private static function fromStatic()
    {
    }

    private function fromNullsafe()
    {
    }

    private function provider()
    {
    }
}

final class Other
{
    public function fromOther()
    {
    }
}
