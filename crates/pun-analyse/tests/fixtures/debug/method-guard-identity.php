<?php

namespace MethodGuardIdentity;

function otherGuards(): void
{
    if (function_exists('method_guard_missing') || function_exists('method_guard_missing')) {
        \method_guard_missing();
    }
    if (defined('stdClass::METHOD_GUARD_MISSING') || defined('stdClass::METHOD_GUARD_MISSING')) {
        echo \stdClass::METHOD_GUARD_MISSING;
    }
    if (defined('METHOD_GUARD_MISSING') || defined('METHOD_GUARD_MISSING')) {
        echo \METHOD_GUARD_MISSING;
    }
    \method_guard_missing();
    echo \stdClass::METHOD_GUARD_MISSING;
    echo \METHOD_GUARD_MISSING;
}

class UnreachableThis
{
    public function check(): void
    {
        if ($this instanceof UnreachableThis && $this instanceof UnreachableThis) {
        } else {
            $this->missing();
        }
        $this->missing();
    }
}
