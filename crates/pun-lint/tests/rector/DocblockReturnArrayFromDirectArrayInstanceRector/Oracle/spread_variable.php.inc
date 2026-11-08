<?php

final class SomeClass
{
    public function run(array $items)
    {
        return [...$items, 1];
    }

    public function constant()
    {
        return [...['a' => 'b'], ...[5, 6]];
    }
}
-----
<?php

final class SomeClass
{
    public function run(array $items)
    {
        return [...$items, 1];
    }

    /**
     * @return array<int|string, int|string>
     */
    public function constant()
    {
        return [...['a' => 'b'], ...[5, 6]];
    }
}
