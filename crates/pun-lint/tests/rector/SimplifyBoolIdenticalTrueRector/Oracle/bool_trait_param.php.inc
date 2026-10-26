<?php

trait BoolTrait
{
    public function f(bool $a)
    {
        return $a === true;
    }
}
-----
<?php

trait BoolTrait
{
    public function f(bool $a)
    {
        return $a;
    }
}
