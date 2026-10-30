<?php
namespace App\Bare;

class C
{
    public function bare(\stdClass $x, bool $c): void
    {
        if ($c) if ($x instanceof \stdClass) echo 1;
        while ($c) if ($x instanceof \stdClass) {
            echo 2;
        }
        echo 3;
    }
}
-----
<?php
namespace App\Bare;

class C
{
    public function bare(\stdClass $x, bool $c): void
    {
        if ($c) echo 1;
        while ($c) echo 2;
        echo 3;
    }
}
