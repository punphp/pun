<?php

final class A
{
    /**
     * @param int $a
     * @param int $b
     */
    public function one(int $a, int $b)
    {
        echo $a;
    }

    /**
     * @param int $b
     */
    public function two(int $b)
    {
    }

    public function three(
        $a,
        $b
    ) {
        echo $a;
    }

    public function __invoke($a) {}

    public static function four($a, $b = [], ...$rest) { return function () use ($a) { $b = 1; }; }
}
-----
<?php

final class A
{
    /**
     * @param int $a
     */
    public function one(int $a)
    {
        echo $a;
    }

    public function two()
    {
    }

    public function three(
        $a
    ) {
        echo $a;
    }

    public function __invoke()
    {
    }

    public static function four($a, $b = []) { return function () use ($a) { $b = 1; }; }
}
