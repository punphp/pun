<?php

final class SomeClass
{
    /**
     * @param array &$x
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
     * @param array &$x
     * @return array<int, string>
     */
    public function run($x = null)
    {
        return ['a', 'b'];
    }
}
