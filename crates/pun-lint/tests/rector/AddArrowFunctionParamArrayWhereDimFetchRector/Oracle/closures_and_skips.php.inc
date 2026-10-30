<?php

final class Rows
{
    public function run(array $rows)
    {
        $closure = array_filter($rows, function ($row) {
            return $row['active'];
        });
        $named = array_filter($rows, 'is_array');
        $objects = array_filter($rows, fn ($row) => $row instanceof \ArrayAccess && $row['active']);
    }
}
-----
<?php

final class Rows
{
    public function run(array $rows)
    {
        $closure = array_filter($rows, function (array $row) {
            return $row['active'];
        });
        $named = array_filter($rows, 'is_array');
        $objects = array_filter($rows, fn ($row) => $row instanceof \ArrayAccess && $row['active']);
    }
}
