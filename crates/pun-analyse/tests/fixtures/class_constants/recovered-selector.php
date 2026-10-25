<?php

namespace RecoveredConstantSelector;

class Receiver { public const PRESENT = 1; }

function recovered(Receiver $receiver): void {
    if ($receiver::) { echo $receiver::PRESENT; }
}
