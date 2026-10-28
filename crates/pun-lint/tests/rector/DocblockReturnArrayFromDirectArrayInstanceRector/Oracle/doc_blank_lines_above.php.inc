<?php

final class SomeClass
{
    /**
     * Text.
     *
     *
     * @return array
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
     * Text.
     *
     *
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
