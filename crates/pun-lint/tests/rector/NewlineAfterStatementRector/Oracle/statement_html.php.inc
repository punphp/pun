<?php

function html($a)
{
    if ($a) {
        return 1;
    }
    return 2;
}
?>
<p>x</p>
<?php
echo html(1);
