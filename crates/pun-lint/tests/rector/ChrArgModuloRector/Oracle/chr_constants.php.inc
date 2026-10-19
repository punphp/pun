<?php

final class ChrConstants
{
    public function run()
    {
        $a = 1;
        $a = 300;
        echo chr($a);
        echo chr(false);
        echo chr(null);
        echo chr(-1.5);
        echo chr(-'a');
    }
}
-----
<?php

final class ChrConstants
{
    public function run()
    {
        $a = 1;
        $a = 300;
        echo chr($a % 256);
        echo chr(false);
        echo chr(null);
        echo chr(-1.5);
        echo chr(-'a');
    }
}
