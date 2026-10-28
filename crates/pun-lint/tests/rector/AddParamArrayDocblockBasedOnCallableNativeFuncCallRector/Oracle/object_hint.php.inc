<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (object $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param object[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (object $item) => $item, $items);
    }
}
