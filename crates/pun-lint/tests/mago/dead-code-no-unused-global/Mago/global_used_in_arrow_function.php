<?php

function lazy(): Closure {
    global $config;
    return fn() => $config;
}
