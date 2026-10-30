<?php

class Base
{
}

final class SomeClass extends Base
{
    public function run(array $items): void
    {
        array_map(fn ((\Countable&\Traversable)|null $item) => $item, $items);
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
     * @param (\Countable & \Traversable)[]|null[] $items
     */
    public function run(array $items): void
    {
        array_map(fn ((\Countable&\Traversable)|null $item) => $item, $items);
    }
}
