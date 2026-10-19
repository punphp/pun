<?php

function f(array $data): void {
    static $secret = 'shh';
    extract($data);
}
