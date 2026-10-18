<?php

function a($value)
{
    switch ($value = 1) {
        case 1:
    }
}
-----
<?php

function a($value)
{
    if (($value = 1) === 1) {
    }
}
