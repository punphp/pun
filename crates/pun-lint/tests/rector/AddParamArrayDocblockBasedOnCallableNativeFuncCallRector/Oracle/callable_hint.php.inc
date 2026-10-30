<?php

class Base
{
}

final class SomeClass extends Base
{
    public function run(array $items): void
    {
        array_map(fn (callable $item) => $item, $items);
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
     * @param callable[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (callable $item) => $item, $items);
    }
}
