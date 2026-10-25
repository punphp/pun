<?php

namespace ThrowsHookTemplates;

/** @template E of \stdClass */
class Hooks
{
    public int $valid {
        /**
         * @template E of \Exception
         * @throws E
         */
        get {
            /** @throws E */
            $value = 1;
            return $value;
        }
    }

    public int $classTemplate {
        /** @throws E */
        get => 1;
    }

    public int $invalid {
        /**
         * @template U of \stdClass
         * @throws U
         */
        set {
            /** @throws U */
            $copy = $value;
        }
    }
}

trait HookTrait
{
    public int $fromTrait {
        /**
         * @template T of \Exception
         * @throws T
         */
        get {
            /** @throws T */
            $value = 1;
            return $value;
        }
    }
}

class UsesTrait { use HookTrait; }
