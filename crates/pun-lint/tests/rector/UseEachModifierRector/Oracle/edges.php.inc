<?php

test('a', function () {
    foreach ($items as $item) foo($item);
    foreach ($items as $item) $item->foo();
    foreach ($items as $item) expect()->toBeString();
});
