<?php

final class A
{
    #[Attr] public function a(
        int $x,
        $y = [
            1,
        ],
    ): ?int {
        if ($x !== null) {
            return $x;
        }

        return null;
    }
}

$f = function (
    $x
) use (&$y) {
    if ($x !== null) {
        return $x;
    }
    return null;
};
-----
<?php

final class A
{
    #[Attr]
    public function a(int $x, $y = [
        1,
    ]): ?int
    {
        return $x;
    }
}

$f = function ($x) use (&$y) {
    return $x;
};
