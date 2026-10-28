<?php

final class SomeClass
{
    /** @return array the list */
    public function run()
    {
        return ['a', 'b'];
    }
}
-----
<?php

final class SomeClass
{
    /** @return array<int, string> the list */
    public function run()
    {
        return ['a', 'b'];
    }
}
