<?php

final class SomeClass
{
    /**
     * @return mixed[]
     *
     *
     * @deprecated
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
     *
     *
     * @deprecated
     */
    public function run()
    {
        return [1, 2];
    }
}
