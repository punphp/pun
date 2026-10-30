<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    public function run(array $keys): array
    {
        $rows = [];
        foreach ($keys as $key) {
            $rows[] = new \ArrayIterator([1]);
        }

        return $rows;
    }
}
-----
<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    /**
     * @return \ArrayIterator<int, int>[]
     */
    public function run(array $keys): array
    {
        $rows = [];
        foreach ($keys as $key) {
            $rows[] = new \ArrayIterator([1]);
        }

        return $rows;
    }
}
