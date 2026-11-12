<?php

function inlineIfComments(bool $flag, int $n)
{
    $flag
        // why
        && $n = 2;
    $flag && $n = 3 /* tail */;
    $flag # hash
        || $n = 4;
    ?><?php $flag && $n = 1 ?>
    <?php
}
