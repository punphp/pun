<?php

function f(): void {
    global $config;
    $name = 'config';
    $$name = 1;
}
