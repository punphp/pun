<?php

final class SomeClass
{
    /**
       Text without star
       @deprecated
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
      Text without star
      @deprecated
     * @return array<int, string>
    */
    public function run()
    {
        return ['a', 'b'];
    }
}
