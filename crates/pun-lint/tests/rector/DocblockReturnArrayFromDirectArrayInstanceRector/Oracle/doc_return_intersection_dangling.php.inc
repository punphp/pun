<?php

final class SomeClass
{
    /**
     * @return int&
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
     * @return int&
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
