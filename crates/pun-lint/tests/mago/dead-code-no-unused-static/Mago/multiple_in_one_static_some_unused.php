<?php

function f(): int {
    static $used = 0, $unused = 0;
    return ++$used;
}
