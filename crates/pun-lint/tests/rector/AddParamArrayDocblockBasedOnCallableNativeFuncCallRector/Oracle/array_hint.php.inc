<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (array $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param mixed[][] $items
     */
    public function run(array $items): void
    {
        array_map(fn (array $item) => $item, $items);
    }
}
