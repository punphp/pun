<?php

enum Suit
{
    case Hearts;
}

class Item
{
}

class Other
{
}

function plain()
{
    $print = function ($suit, $item) {
        var_dump($suit, $item);
    };

    $print(Suit::Hearts, item: new Item());
}

class Kinds
{
    public function self()
    {
        $print = function ($a, $b, $c) {
            var_dump($a, $b, $c);
        };

        $print(new static(), $this, new Item());
        $print(new static(), $this, new Item());
    }

    public function unpacked(array $items)
    {
        $print = function ($a, $b) {
            var_dump($a, $b);
        };

        $print(new Item(), ...$items);
    }

    public function reassigned()
    {
        $print = function ($a) {
            var_dump($a);
        };
        $print = function ($b) {
            var_dump($b);
        };

        $print(new Other());
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

    public function partial()
    {
        $print = function ($a, $b) {
            var_dump($a, $b);
        };

        $print(new Item());
        $print(new Item(), new Other());
    }
}
-----
<?php

enum Suit
{
    case Hearts;
}

class Item
{
}

class Other
{
}

function plain()
{
    $print = function (\Suit $suit, \Item $item) {
        var_dump($suit, $item);
    };

    $print(Suit::Hearts, item: new Item());
}

class Kinds
{
    public function self()
    {
        $print = function ($a, $b, \Item $c) {
            var_dump($a, $b, $c);
        };

        $print(new static(), $this, new Item());
        $print(new static(), $this, new Item());
    }

    public function unpacked(array $items)
    {
        $print = function (\Item $a, $b) {
            var_dump($a, $b);
        };

        $print(new Item(), ...$items);
    }

    public function reassigned()
    {
        $print = function ($a) {
            var_dump($a);
        };
        $print = function (\Other $b) {
            var_dump($b);
        };

        $print(new Other());
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

    public function partial()
    {
        $print = function (\Item $a, \Other $b) {
            var_dump($a, $b);
        };

        $print(new Item());
        $print(new Item(), new Other());
    }
}
