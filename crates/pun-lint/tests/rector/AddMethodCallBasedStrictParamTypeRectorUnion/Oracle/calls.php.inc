<?php

namespace Probe;

use Closure;

final class Calls
{
    public function run(array $items, \DateTime $date, ?callable $check, Closure $closure)
    {
        $this->order(2, []);
        $this->order([1], 'a');
        $this->order(new \DateTime(), 1.5);
        $this->defaults('x');
        $this->defaults(null, 3);
        self::staticCall($date);
        static::staticCall($date);
        $this->callables($check, $closure);
        $this->unpacked(...$items);
        $this->named(b: 1, a: 'x');
        $copy = $this;
        $copy->onVariable(true);
        $this->onVariable(false);
        array_map(fn () => $this->inClosure(1), []);
        $this->self($this);
    }

    private function order($first, $second)
    {
    }

    private function defaults($a = 'yes', $b = 2)
    {
    }

    private static function staticCall($date)
    {
    }

    private function callables($check, $closure)
    {
    }

    private function unpacked($a, $b)
    {
    }

    private function named($a, $b)
    {
    }

    private function onVariable($flag)
    {
    }

    private function inClosure($number)
    {
    }

    private function self($calls)
    {
    }

    protected function protectedNotCalled($a)
    {
    }

    final protected function finalProtected($a)
    {
    }

    public function callFinal()
    {
        $this->finalProtected(1);
        $this->magic(1);
    }

    public function __call($name, $arguments)
    {
    }
}
-----
<?php

namespace Probe;

use Closure;

final class Calls
{
    public function run(array $items, \DateTime $date, ?callable $check, Closure $closure)
    {
        $this->order(2, []);
        $this->order([1], 'a');
        $this->order(new \DateTime(), 1.5);
        $this->defaults('x');
        $this->defaults(null, 3);
        self::staticCall($date);
        static::staticCall($date);
        $this->callables($check, $closure);
        $this->unpacked(...$items);
        $this->named(b: 1, a: 'x');
        $copy = $this;
        $copy->onVariable(true);
        $this->onVariable(false);
        array_map(fn () => $this->inClosure(1), []);
        $this->self($this);
    }

    private function order(int|array|\DateTime $first, array|string|float $second)
    {
    }

    private function defaults(?string $a = 'yes', int $b = 2)
    {
    }

    private static function staticCall(\DateTime $date)
    {
    }

    private function callables(?callable $check, \Closure $closure)
    {
    }

    private function unpacked($a, $b)
    {
    }

    private function named(int $a, string $b)
    {
    }

    private function onVariable(bool $flag)
    {
    }

    private function inClosure(int $number)
    {
    }

    private function self(\Probe\Calls $calls)
    {
    }

    protected function protectedNotCalled($a)
    {
    }

    final protected function finalProtected(int $a)
    {
    }

    public function callFinal()
    {
        $this->finalProtected(1);
        $this->magic(1);
    }

    public function __call($name, $arguments)
    {
    }
}
