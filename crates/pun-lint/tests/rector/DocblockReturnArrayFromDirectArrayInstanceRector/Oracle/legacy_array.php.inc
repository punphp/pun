<?php

final class LegacyArray
{
    public function run()
    {
        return array('a', 'b');
    }

    public function empty()
    {
        return array();
    }
}
-----
<?php

final class LegacyArray
{
    /**
     * @return array<int, string>
     */
    public function run()
    {
        return array('a', 'b');
    }

    /**
     * @return array{}
     */
    public function empty()
    {
        return array();
    }
}
