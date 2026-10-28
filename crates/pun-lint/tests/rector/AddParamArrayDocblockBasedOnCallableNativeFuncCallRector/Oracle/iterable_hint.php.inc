<?php

class Base
{
}

final class SomeClass extends Base
{
    public function run(array $items): void
    {
        array_map(fn (iterable $item) => $item, $items);
    }
}
-----
<?php

class Base
{
}

final class SomeClass extends Base
{
    /**
     * @param iterable[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (iterable $item) => $item, $items);
    }
}
