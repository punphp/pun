<?php

namespace Oracle\AlwaysTrueArrays;

if (1 === 1) {
    echo 'top';
}

final class Lists
{
    public function plain(array $items)
    {
        if ($items || true) {
            echo 1;
        }
    }

    public function nonEmpty(array $items)
    {
        if ($items === []) {
            return 0;
        }
        if ($items || true) {
            echo 2;
        }
        return 1;
    }

    public function known()
    {
        $list = [1, 2];
        if ($list || true) {
            echo 3;
        }
        $map = ['a' => 1];
        if ($map || true) {
            echo 4;
        }
    }

    public function assigns()
    {
        $flag = true;
        if ($flag = true) {
            echo 5;
        }
        if (($flag = true) === true) {
            echo 6;
        }
        if (($flag = true) || true) {
            echo 7;
        }
        if (true || ($flag = true)) {
            echo 8;
        }
    }
}
-----
<?php

namespace Oracle\AlwaysTrueArrays;

echo 'top';

final class Lists
{
    public function plain(array $items)
    {
        echo 1;
    }

    public function nonEmpty(array $items)
    {
        if ($items === []) {
            return 0;
        }
        echo 2;
        return 1;
    }

    public function known()
    {
        $list = [1, 2];
        echo 3;
        $map = ['a' => 1];
        echo 4;
    }

    public function assigns()
    {
        $flag = true;
        if ($flag = true) {
            echo 5;
        }
        if (($flag = true) === true) {
            echo 6;
        }
        if (($flag = true) || true) {
            echo 7;
        }
        if (true || ($flag = true)) {
            echo 8;
        }
    }
}
