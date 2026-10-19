<?php

function a($x)
{
    if ($x)
        while ($x) {
        }
}
-----
<?php

function a($x)
{
    if ($x) {
    }
}
