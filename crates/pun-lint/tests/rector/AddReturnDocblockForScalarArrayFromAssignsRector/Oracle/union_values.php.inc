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
     * @return 'a'[]|int[]
     */
    public function run($flag)
    {
        $v = [];
        $v[] = $flag ? 1 : 'a';

        return $v;
    }
}
