<?php

namespace Probe;

final class Calls
{
    public function run(\Unknown\Thing $thing, array $args, ?int $maybe, array $list, string $method, self $other)
    {
        $this->spread(...$args);
        $this->__get('name');
        $this->{$method}(1);
        $this->mixedFirst($args['x']);
        $this->mixedFirst(1);
        $this->unknown(new \Unknown\Thing());
        Calls::named(1);
        $other::named(2);
        $this->nullable($maybe);
        $this->listed($list);
        $this->badDefault(1);
        $this->constDefault(1);
        $this->nullDefault(1);
        $this->unknownParam($thing);
        $this->numericKey(['1' => 1]['1']);
        $this->stringKey(pathinfo('a')["dir$method"]);
    }

    private function spread($value)
    {
    }

    private function __get($name)
    {
        return $name;
    }

    private function mixedFirst($value)
    {
    }

    private function unknown($value)
    {
    }

    private static function named($value)
    {
    }

    private function nullable($value = null)
    {
    }

    private function listed($value = [])
    {
    }

    private function badDefault($value = 'a')
    {
    }

    private function numericKey($value)
    {
    }

    private function stringKey($value)
    {
    }

    private function constDefault($value = UNKNOWN_CONST)
    {
    }

    private function nullDefault($value = null)
    {
    }

    private function unknownParam($value)
    {
    }
}
-----
<?php

namespace Probe;

final class Calls
{
    public function run(\Unknown\Thing $thing, array $args, ?int $maybe, array $list, string $method, self $other)
    {
        $this->spread(...$args);
        $this->__get('name');
        $this->{$method}(1);
        $this->mixedFirst($args['x']);
        $this->mixedFirst(1);
        $this->unknown(new \Unknown\Thing());
        Calls::named(1);
        $other::named(2);
        $this->nullable($maybe);
        $this->listed($list);
        $this->badDefault(1);
        $this->constDefault(1);
        $this->nullDefault(1);
        $this->unknownParam($thing);
        $this->numericKey(['1' => 1]['1']);
        $this->stringKey(pathinfo('a')["dir$method"]);
    }

    private function spread($value)
    {
    }

    private function __get($name)
    {
        return $name;
    }

    private function mixedFirst($value)
    {
    }

    private function unknown($value)
    {
    }

    private static function named(int $value)
    {
    }

    private function nullable(?int $value = null)
    {
    }

    private function listed(array $value = [])
    {
    }

    private function badDefault($value = 'a')
    {
    }

    private function numericKey(int $value)
    {
    }

    private function stringKey(string $value)
    {
    }

    private function constDefault(int $value = UNKNOWN_CONST)
    {
    }

    private function nullDefault($value = null)
    {
    }

    private function unknownParam($value)
    {
    }
}
