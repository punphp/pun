<?php

final class SomeClass
{
    /**
     * @return mixed[]-desc
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
     * @return mixed[]-desc
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
