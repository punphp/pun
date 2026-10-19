<?php

function f(): int {
    global $used, $unused;
    return $used;
}
