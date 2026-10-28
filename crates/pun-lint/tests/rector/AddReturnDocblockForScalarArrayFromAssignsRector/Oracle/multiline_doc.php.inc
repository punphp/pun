<?php

abstract class SomeClass
{
    /**
     * @param array<
     *     int
     * > $flag
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
     * @param array<
     *     int
     * > $flag
     * @return int[]
     */
    public function run($flag)
    {
        $v = [];
        $v[] = 1;

        return $v;
    }
}
