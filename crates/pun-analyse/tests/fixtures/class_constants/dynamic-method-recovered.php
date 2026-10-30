<?php

function recoveredMethodSelectors(\stdClass $receiver): void {
    if ($receiver->()::MISSING) {}
    if (\stdClass::()::MISSING) {}
}
