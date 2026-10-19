<?php

function outer(): void {
    global $config;

    function inner(): void {
        echo $config; // separate scope, does not satisfy outer global
    }
}
