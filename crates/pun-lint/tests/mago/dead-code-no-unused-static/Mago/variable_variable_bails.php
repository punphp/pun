<?php

function f(): void {
    static $secret = 'shh';
    $name = 'thing';
    $$name = 1;
}
