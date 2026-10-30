<?php

class Base
{
}

final class SomeClass extends Base
{
    public function run(array $items): void
    {
        array_map(fn (parent $item) => $item, $items);
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
     * @param static[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (parent $item) => $item, $items);
    }
}
