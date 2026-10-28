<?php

final class SomeClass
{
    /**
       * Misindented
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
     * Misindented
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
