<?php

final class SomeClass
{
    /**
     * @param int
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
     * @param int
     * @return array<int, string>
     */
    public function run($x = null)
    {
        return ['a', 'b'];
    }
}
