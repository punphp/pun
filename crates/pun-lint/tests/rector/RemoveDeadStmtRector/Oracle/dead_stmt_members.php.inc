<?php

class Members
{
    public function f($a, $b)
    {
        -$a->$b;
        -$a->{$b . foo()};
    }
}
-----
<?php

class Members
{
    public function f($a, $b)
    {
        foo();
    }
}
