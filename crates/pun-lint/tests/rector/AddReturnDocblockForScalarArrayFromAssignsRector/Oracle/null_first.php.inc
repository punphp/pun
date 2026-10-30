<?php

final class SomeClass
{
    public function run()
    {
        $v = null;
        $v = [];
        $v[] = 'a';

        return $v;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @return string[]
     */
    public function run()
    {
        $v = null;
        $v = [];
        $v[] = 'a';

        return $v;
    }
}
