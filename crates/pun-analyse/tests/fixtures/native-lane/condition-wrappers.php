<?php
function wrapped(bool $flag): void {
    if (($flag)) {
        $flag; // native
    } else {
        $flag; // native
    }
    if (!($flag)) {
        $flag; // native
    } else {
        $flag; // native
    }
    ($flag) ? 1 : 2; // native
    $flag; // native
}
