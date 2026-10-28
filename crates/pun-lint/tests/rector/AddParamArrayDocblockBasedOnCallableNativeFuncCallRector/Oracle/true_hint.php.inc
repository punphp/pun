<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (true $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param true[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (true $item) => $item, $items);
    }
}
