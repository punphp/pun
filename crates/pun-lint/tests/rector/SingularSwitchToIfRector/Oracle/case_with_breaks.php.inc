<?php

function a($value)
{
    foreach ($value as $v) {
        // before the switch
        switch ($v) {
            case 'a':
                // first
                echo 1;
                if ($v) {
                    break;
                }
                break 2;
            // last
        }
        switch ($v):
            case $v + 1:
                echo 2;
                break 3;
                break 1;
                echo 3;
        endswitch;
    }
}
-----
<?php

function a($value)
{
    foreach ($value as $v) {
        // before the switch
        switch ($v) {
            case 'a':
                // first
                echo 1;
                if ($v) {
                    break;
                }
                break 2;
            // last
        }
        if ($v === $v + 1) {
            echo 2;
            break 2;
            echo 3;
        }
    }
}
