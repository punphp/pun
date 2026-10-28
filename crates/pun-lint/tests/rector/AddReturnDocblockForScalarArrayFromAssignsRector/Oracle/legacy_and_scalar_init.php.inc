<?php

abstract class SomeClass
{
    public function run($flag)
    {
        $v = array();
        $v[] = 1;
        $w = 5;

        return $v;
    }
}
-----
<?php

abstract class SomeClass
{
    /**
     * @return int[]
     */
    public function run($flag)
    {
        $v = array();
        $v[] = 1;
        $w = 5;

        return $v;
    }
}
