<?php

final class SomeClass
{
    /**
     * @var int &$x
     */
    public function run()
    {
        return [1, 2];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @var int &$x
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
