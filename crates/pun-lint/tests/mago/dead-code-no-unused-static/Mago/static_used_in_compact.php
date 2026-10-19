<?php

function snapshot(): array {
    static $latest = null;
    return compact('latest');
}
