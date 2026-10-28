<?php

abstract class SomeClass
{
    public function run($flag)
    {
        $v = [];
        $v[] = (string) $flag;
        $v[] = (string) $flag;

        return $v;
    }
}
-----
<?php

abstract class SomeClass
{
    /**
     * @return string[]
     */
    public function run($flag)
    {
        $v = [];
        $v[] = (string) $flag;
        $v[] = (string) $flag;

        return $v;
    }
}
