<?php

final class SomeClass
{
    /**
     * @deprecated
     *
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @deprecated
     *
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
