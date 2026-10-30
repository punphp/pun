<?php
namespace BlockPollution;

function nativeTypes(): void {
    $existing = 1;
    {
        $existing = 2;
        $created = 3;
        $existing; // native
    }
    $existing; // native
    $created; // native
    $outside = 4;
    $outside; // native
}
