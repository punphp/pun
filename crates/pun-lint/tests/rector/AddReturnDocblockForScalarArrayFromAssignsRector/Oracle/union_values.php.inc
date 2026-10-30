<?php

abstract class SomeClass
{
    public function run($flag)
    {
        $v = [];
        $v[] = $flag ? 1 : 'a';

        return $v;
    }
}
-----
<?php

abstract class SomeClass
{
    /**
     * @return int[]|'a'[]
     */
    public function run($flag)
    {
        $v = [];
        $v[] = $flag ? 1 : 'a';

        return $v;
    }
}
