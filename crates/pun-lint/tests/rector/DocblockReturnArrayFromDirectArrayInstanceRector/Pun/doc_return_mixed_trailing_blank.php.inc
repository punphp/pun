<?php

final class SomeClass
{
    /**
     * @return mixed[]
     *
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
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
