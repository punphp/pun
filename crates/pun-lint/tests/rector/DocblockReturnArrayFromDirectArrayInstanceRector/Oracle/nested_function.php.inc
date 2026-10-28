<?php

function outer()
{
    /**
     * inner
     */
    function inner()
    {
        return ['k' => 1.5];
    }
}
-----
<?php

function outer()
{
    /**
     * inner
     * @return array<string, float>
     */
    function inner()
    {
        return ['k' => 1.5];
    }
}
