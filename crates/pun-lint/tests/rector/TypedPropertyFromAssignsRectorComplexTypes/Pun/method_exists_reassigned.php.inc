<?php

namespace App;

// Rector writes `private object|string|null $reassigned = null;`: PHPStan narrows `method_exists($r, ...)` from the
// type `$r` has there. Mago types the narrowed variable `has-method<'run'>` whatever it was before, and Pun only
// recovers that type for a parameter its method never assigns, so it leaves this property alone.
final class MethodExistsReassigned
{
    private $reassigned;

    public function a($r)
    {
        $r = $r ?: null;
        if (method_exists($r, 'run')) {
            $this->reassigned = $r;
        }
    }
}
