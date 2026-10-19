<?php

function snapshot(): array {
    global $config;
    return compact('config');
}
