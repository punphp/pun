<?php

abstract class SomeClass
{
    /**
     * @return <
     */
    public function run($flag)
    {
        $v = [];
        $v[] = 1;

        return $v;
    }
}
-----
<?php

abstract class SomeClass
{
    /**
     * @return <
     * @return int[]
     */
    public function run($flag)
    {
        $v = [];
        $v[] = 1;

        return $v;
    }
}
