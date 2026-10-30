<?php

final class Maps
{
    /**
     * @param  list<int>  $items
     * @param  list<string>  $names
     */
    public function run(array $items, array $names)
    {
        $trimmed = array_map('trim', $names);
        $arrow = array_map(fn ($item) => $item, $items);
        $pairs = array_map(function ($item) {
            return $item;
        }, $items, $names);
        $literal = array_map(function ($item) {
            return $item;
        }, [1, 2]);
    }
}
-----
<?php

final class Maps
{
    /**
     * @param  list<int>  $items
     * @param  list<string>  $names
     */
    public function run(array $items, array $names)
    {
        $trimmed = array_map('trim', $names);
        $arrow = array_map(fn ($item) => $item, $items);
        $pairs = array_map(function (int $item) {
            return $item;
        }, $items, $names);
        $literal = array_map(function ($item) {
            return $item;
        }, [1, 2]);
    }
}
