<?php

function check(object $o, array $classes)
{
    return is_a($o, ...$classes);
}
