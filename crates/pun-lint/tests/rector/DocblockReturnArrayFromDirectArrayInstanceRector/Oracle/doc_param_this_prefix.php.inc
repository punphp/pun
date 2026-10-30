<?php

final class SomeClass
{
    /**
     * @param $thisValue
     * @param $this $other
     */
    public function run($thisValue, $other)
    {
        return [1, 2];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param $thisValue
     * @param $this $other
     * @return array<int, int>
     */
    public function run($thisValue, $other)
    {
        return [1, 2];
    }
}
