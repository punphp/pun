<?php
#[\self] class EarlierName {}
#[\Deprecated]
const A = new class {
    function first(void $value) {}
}, B = new class {
    function second(void $value) {}
};
#[\Deprecated]
#[\Deprecated]
const C = 1, D = new class {
    function third(void $value) {}
};
