<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (string $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param string[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (string $item) => $item, $items);
    }
}
