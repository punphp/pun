<?php

namespace InternalLib\Sub {
    /** @internal */
    function helper(): int
    {
        return 1;
    }

    function open(): int
    {
        return 2;
    }

    /** @internal */
    class Hidden
    {
        public const VALUE = 1;
    }

    class Visible
    {
        /** @internal */
        public const SECRET = 1;
        public const PLAIN = 2;
    }

    /** @internal */
    interface HiddenInterface
    {
        public const FROM_INTERFACE = 1;
    }

    class Child extends Visible implements HiddenInterface
    {
        public function inside(): void
        {
            echo self::SECRET;
            echo helper();
        }
    }

    /** @internal */
    enum HiddenEnum
    {
        case A;
        public const E = 1;
    }
}

namespace InternalLib {
    echo Sub\helper();
    echo Sub\Visible::SECRET;
}

namespace Consumer {
    use InternalLib\Sub;
    use InternalLib\Sub\Child;

    echo Sub\helper();
    echo Sub\open();
    $f = Sub\helper(...);
    echo Sub\Hidden::VALUE;
    echo Sub\Visible::SECRET;
    echo Sub\Visible::PLAIN;
    echo Child::SECRET;
    echo Child::FROM_INTERFACE;
    echo Sub\HiddenInterface::FROM_INTERFACE;
    echo Sub\HiddenEnum::E;
    echo Sub\HiddenEnum::A;
    echo Sub\Hidden::class;

    class Extending extends Child
    {
        public function use(): void
        {
            echo self::SECRET;
            echo static::PLAIN;
            echo parent::SECRET;
            echo $this::SECRET;
        }
    }
}

namespace {
    /** @internal */
    function global_helper(): void
    {
    }

    /** @internal */
    class GlobalHidden
    {
        public const G = 1;
    }
}

namespace Elsewhere {
    global_helper();
    echo \GlobalHidden::G;
}
