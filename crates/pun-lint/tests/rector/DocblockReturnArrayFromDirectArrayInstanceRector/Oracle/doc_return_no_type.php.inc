<?php

final class SomeClass
{
    /**
     * @return
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
     * @return
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
