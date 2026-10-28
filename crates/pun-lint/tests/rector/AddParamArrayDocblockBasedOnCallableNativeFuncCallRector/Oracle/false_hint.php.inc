<?php

class Base
{
}

final class SomeClass extends Base
{
    public function run(array $items): void
    {
        array_map(fn (false $item) => $item, $items);
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
     * @param false[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (false $item) => $item, $items);
    }
}
