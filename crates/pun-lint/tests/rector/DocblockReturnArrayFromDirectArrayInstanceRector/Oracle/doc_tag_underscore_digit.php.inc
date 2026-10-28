<?php

final class SomeClass
{
    /**
     * @_1 text
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
     * @_1 text
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
