<?php

final class SomeClass
{
    /**
     * @deprecated
     * More text.
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
     * More text.
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
