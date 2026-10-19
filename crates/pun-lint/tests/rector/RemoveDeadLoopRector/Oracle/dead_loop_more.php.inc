<?php

function a($items, $x)
{
    while ($x):
    endwhile;
    for ($i = 0; $i < 10; $i++):
    endfor;
    foreach ($items as $item):
    endforeach;
    while ($x):
        foo();
    endwhile;
    while ($x) {
        foo();
    }
}

class A
{
    public int $value {
        get {
            while ($this->x) {
            }
            return 1;
        }
    }
}
-----
<?php

function a($items, $x)
{
    while ($x):
        foo();
    endwhile;
    while ($x) {
        foo();
    }
}

class A
{
    public int $value {
        get {
            return 1;
        }
    }
}
