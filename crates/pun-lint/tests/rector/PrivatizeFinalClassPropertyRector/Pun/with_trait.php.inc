<?php

// Pun: a class that uses a trait stays, since the syntax cannot see whether the trait uses the property (Rector
// changes it when the trait cannot be loaded, as here).
final class WithTrait
{
    use T;

    protected $x;
}
