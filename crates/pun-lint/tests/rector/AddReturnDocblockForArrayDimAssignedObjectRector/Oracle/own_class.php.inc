<?php

namespace App;

final class Row
{
    public function __construct(public string $key) {}
}

final class OwnClass
{
    /**
     * @param string[] $keys
     */
    public function rows(array $keys): array
    {
        $rows = [];
        foreach ($keys as $key) {
            $rows[] = new Row($key);
        }

        return $rows;
    }
}
-----
<?php

namespace App;

final class Row
{
    public function __construct(public string $key) {}
}

final class OwnClass
{
    /**
     * @param string[] $keys
     * @return \App\Row[]
     */
    public function rows(array $keys): array
    {
        $rows = [];
        foreach ($keys as $key) {
            $rows[] = new Row($key);
        }

        return $rows;
    }
}
