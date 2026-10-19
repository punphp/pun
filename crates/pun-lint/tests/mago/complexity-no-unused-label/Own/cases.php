<?php

start:
used:
function f() {
    inner:
    goto used;
    again:
    goto again;
}
goto start;
