<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (float $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param float[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (float $item) => $item, $items);
    }
}
