<?php

function a()
{
    $a = 1;
    $a->b = 2;
    $c = 3;
    $c->x();
    $c->y();
    $d = 4;
    $this->e = 5;
    $this->e = 6;
    $this->f->g = 7;
    $h->i->j = 8;
    $h->k = 9;
    $l = 10;
}

class C
{
    public function m()
    {
        $a = 1;
        $a = 2;

        $b = 3;
        $c = [
            1,
        ];
        $c[] = 1;
        $d = 2;
        $e = 1;
        $e = 2;
        $f = function () {
            $x = 1;
            $x = 2;
            $y = 3;
        };
        $g = 1;
        Foo::$s = 1;
        Foo::$s = 2;
        $t = 1;
        $this->x();
        $this->x();
        $u = 1;
        foo()->a = 1;
        foo()->b = 2;
        $v = 1;
        $w?->x();
        $w?->y();
        $z = 1;
    }
}
-----
<?php

function a()
{
    $a = 1;
    $a->b = 2;

    $c = 3;
    $c->x();
    $c->y();

    $d = 4;
    $this->e = 5;
    $this->e = 6;

    $this->f->g = 7;
    $h->i->j = 8;
    $h->k = 9;

    $l = 10;
}

class C
{
    public function m()
    {
        $a = 1;
        $a = 2;

        $b = 3;
        $c = [
            1,
        ];
        $c[] = 1;
        $d = 2;
        $e = 1;
        $e = 2;

        $f = function () {
            $x = 1;
            $x = 2;

            $y = 3;
        };
        $g = 1;
        Foo::$s = 1;
        Foo::$s = 2;

        $t = 1;
        $this->x();
        $this->x();
        $u = 1;
        foo()->a = 1;
        foo()->b = 2;

        $v = 1;
        $w?->x();
        $w?->y();
        $z = 1;
    }
}
