<?php

use PHPUnit\Framework\Attributes\DataProvider;

final class Y1Test extends \PHPUnit\Framework\TestCase
{
    public static function provider(): iterable
    {
        yield 'case' => ["a" => 1, 2, 3];
        yield from [['x' => 1]];
    }

    public static function second(): array
    {
        $sets = [['p' => 1, 'q' => 2]];
        $sets = [['p' => 1, 'q' => 2, 'r' => 3]];
        return [($sets), ['one', 'b' => 2], ...[]];
    }

    #[DataProvider(methodName: 'second')]
    public function testA(int $first, int $second): void
    {
    }

    #[DataProvider('PROVIDER')]
    public function testB(int $other, int $second, int $third): void
    {
    }

    #[DataProvider('second')]
    protected function testC(int $first): void
    {
    }
}
-----
<?php

use PHPUnit\Framework\Attributes\DataProvider;

final class Y1Test extends \PHPUnit\Framework\TestCase
{
    public static function provider(): iterable
    {
        yield 'case' => ["other" => 1, "second" => 2, "third" => 3];
        yield from [['x' => 1]];
    }

    public static function second(): array
    {
        $sets = [['p' => 1, 'q' => 2]];
        $sets = [['first' => 1, 'second' => 2, 'r' => 3]];
        return [($sets), ["first" => 'one', 'second' => 2], ...[]];
    }

    #[DataProvider(methodName: 'second')]
    public function testA(int $first, int $second): void
    {
    }

    #[DataProvider('PROVIDER')]
    public function testB(int $other, int $second, int $third): void
    {
    }

    #[DataProvider('second')]
    protected function testC(int $first): void
    {
    }
}
