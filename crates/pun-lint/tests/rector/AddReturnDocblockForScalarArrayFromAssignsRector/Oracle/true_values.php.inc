<?php

abstract class SomeClass
{
    public function run($flag)
    {
        $v = [];
        $v[] = true;
        $v[] = true;

        return $v;
    }
}
-----
<?php

abstract class SomeClass
{
    /**
     * @return true[]
     */
    public function run($flag)
    {
        $v = [];
        $v[] = true;
        $v[] = true;

        return $v;
    }
}
