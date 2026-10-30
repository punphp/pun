<?php

final class SomeClass
{
    /**
     * @param $x
     */
    public function run($x = null)
    {
        return ['a', 'b'];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param $x
     * @return array<int, string>
     */
    public function run($x = null)
    {
        return ['a', 'b'];
    }
}
