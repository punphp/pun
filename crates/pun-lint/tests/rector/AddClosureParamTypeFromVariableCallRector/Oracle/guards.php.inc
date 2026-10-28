<?php

class Item
{
}

class Other
{
}

abstract class Kinds
{
    abstract public function nothing();

    public function union(bool $flag)
    {
        $print = function ($a) {
            var_dump($a);
        };

        strlen('x');
        $print($flag ? new Item() : new Other());
    }

    public function nested()
    {
        return function () {
            $print = function ($a) {
                var_dump($a);
            };

            $print(new Item());
        };
    }
}
-----
<?php

class Item
{
}

class Other
{
}

abstract class Kinds
{
    abstract public function nothing();

    public function union(bool $flag)
    {
        $print = function ($a) {
            var_dump($a);
        };

        strlen('x');
        $print($flag ? new Item() : new Other());
    }

    public function nested()
    {
        return function () {
            $print = function (\Item $a) {
                var_dump($a);
            };

            $print(new Item());
        };
    }
}
