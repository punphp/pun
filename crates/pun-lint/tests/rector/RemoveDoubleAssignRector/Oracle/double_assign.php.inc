<?php

function a()
{
    $a = 1;
    $a = 2;
    $a = 3;
    // c
    $this->b = [];
    $this->b = [1];
    $c = foo();
    $c = 2;
    $d = count($x);
    $d = 3;
    $e = 1;
    $e = $e + 1;
    $f[0] = 1;
    $f[0] = 2;
    $this->g()->h = 1;
    $this->g()->h = 2;
    self::$i = 1;
    self::$i = 2;
    $this->j = 1;
    $this->j = $this->k();
    try {
        $l = 1;
        $l = m();
    } catch (E $e) {
    }
    $n = 1; $n = 2;
}
-----
<?php

function a()
{
    $a = 3;
    $this->b = [1];
    $c = foo();
    $c = 2;
    $d = 3;
    $e = 1;
    $e = $e + 1;
    $f[0] = 1;
    $f[0] = 2;
    $this->g()->h = 1;
    $this->g()->h = 2;
    self::$i = 2;
    $this->j = 1;
    $this->j = $this->k();
    try {
        $l = 1;
        $l = m();
    } catch (E $e) {
    } $n = 2;
}
