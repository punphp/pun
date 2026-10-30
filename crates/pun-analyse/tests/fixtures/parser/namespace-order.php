<?php
namespace OrderOuter {
    namespace First {
        function first(void $first) {}
        namespace Deep {
            try {}
        }
    }
    function between(void $between) {}
    namespace Second {
        class C { public int $p { get() => 1; } }
    }
    function after(void $after) {}
}
