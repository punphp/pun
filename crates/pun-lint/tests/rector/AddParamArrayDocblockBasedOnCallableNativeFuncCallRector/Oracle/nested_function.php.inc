<?php

final class SomeClass
{
    public function run(array $items): void
    {
        $f = function () use ($items) {
            array_map(fn (int $item) => $item, $items);
        };
        $o = new class {
        };
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param int[] $items
     */
    public function run(array $items): void
    {
        $f = function () use ($items) {
            array_map(fn (int $item) => $item, $items);
        };
        $o = new class {
        };
    }
}
