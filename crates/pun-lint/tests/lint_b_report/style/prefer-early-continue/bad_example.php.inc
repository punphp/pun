<?php

for ($i = 0; $i < 10; $i++) {
    if ($condition) {
        doSomething();
    }
}
-----
4:5: Consider using early continue pattern to reduce nesting.
