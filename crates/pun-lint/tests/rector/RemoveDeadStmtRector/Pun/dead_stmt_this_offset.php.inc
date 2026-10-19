<?php

class ThisOffset
{
    public function f()
    {
        -$this['a'];
    }
}
-----
<?php

class ThisOffset
{
    public function f()
    {
        $this['a'];
    }
}
