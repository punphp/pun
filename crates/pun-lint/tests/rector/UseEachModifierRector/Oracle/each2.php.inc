<?php

test('a', function () {
    foreach ($items as $item) expect($item->x)->toBeString();
});
