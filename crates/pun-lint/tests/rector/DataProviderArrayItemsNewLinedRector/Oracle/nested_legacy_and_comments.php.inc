<?php

final class P1Test extends \PHPUnit\Framework\TestCase
{
    public function provideA(): array
    {
        return array(['a', [
            1,
        ]], /* c */ ['b'],
            ['c'], ['d']);
    }

    public function provideB(): iterable
    {
        $f = function () {
            return [1, 2];
        };
        if (true) {
                return [1,
                    2, 3];
        }
        return [];
    }

    protected function provideC(): array
    {
        return [1, 2];
    }

    #[\PHPUnit\Framework\Attributes\Test]
    public function provideD(): array
    {
        return [1, 2];
    }

    public function TestE(): array
    {
        return ['k' => 1, ...$x];
    }
}
-----
<?php

final class P1Test extends \PHPUnit\Framework\TestCase
{
    public function provideA(): array
    {
        return [
            ['a', [
                1,
            ]],
            /* c */
            ['b'],
            ['c'],
            ['d'],
        ];
    }

    public function provideB(): iterable
    {
        $f = function () {
            return [
                1,
                2,
            ];
        };
        if (true) {
                return [
                    1,
                    2,
                    3,
                ];
        }
        return [];
    }

    protected function provideC(): array
    {
        return [1, 2];
    }

    #[\PHPUnit\Framework\Attributes\Test]
    public function provideD(): array
    {
        return [1, 2];
    }

    public function TestE(): array
    {
        return [
            'k' => 1,
            ...$x,
        ];
    }
}
