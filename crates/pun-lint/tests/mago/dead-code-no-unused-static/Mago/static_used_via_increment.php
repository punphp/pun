<?php

function next_id(): int {
    static $counter = 0;
    return ++$counter;
}
