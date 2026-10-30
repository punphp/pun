<?php

class Other
{
    /**
     * @var array<string, int>
     */
    public static array $items = [];
}

final class Keys
{
    public function emptyLocal($key)
    {
        $map = [];
        $map[$key] = 1;

        return $map;
    }

    public function otherStatic($key)
    {
        Other::$items[$key] = 1;
    }

    public function dynamicStatic($key, Other $other)
    {
        $other::$items[$key] = 1;
    }

    public function listLocal($key)
    {
        $list = [1, 2];
        $list[$key] = 3;

        return $list;
    }
}
-----
<?php

class Other
{
    /**
     * @var array<string, int>
     */
    public static array $items = [];
}

final class Keys
{
    public function emptyLocal($key)
    {
        $map = [];
        $map[$key] = 1;

        return $map;
    }

    public function otherStatic(string $key)
    {
        Other::$items[$key] = 1;
    }

    public function dynamicStatic(string $key, Other $other)
    {
        $other::$items[$key] = 1;
    }

    public function listLocal(int $key)
    {
        $list = [1, 2];
        $list[$key] = 3;

        return $list;
    }
}
