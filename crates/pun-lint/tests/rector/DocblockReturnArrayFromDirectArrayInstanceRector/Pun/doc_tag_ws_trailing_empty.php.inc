<?php

final class SomeClass
{
    /**
     * @deprecated  
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
     * @deprecated 
     *
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
