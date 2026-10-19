<?php

function lazy(): Closure {
    global $config;
    return function () use ($config) {
        return $config;
    };
}
