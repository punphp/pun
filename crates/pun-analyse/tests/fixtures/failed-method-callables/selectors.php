<?php
// Ordinary calls and properties retain their existing selector validation.
class SelectorTarget {}
function ordinaryControls(SelectorTarget $receiver): mixed {
    $name = 123;
    $receiver->{$name}();
    return $receiver->{$name};
}
