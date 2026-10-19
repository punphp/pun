<?php

$a = function ($x) {
    if ($x) {
        return true;
    }
    return false;
};

$b = function () {
    return false;
};

$c = function ($x) {
    if ($x) {
        return 1;
    }
    return false;
};
-----
<?php

$a = function ($x): bool {
    if ($x) {
        return true;
    }
    return false;
};

$b = function (): false {
    return false;
};

$c = function ($x): int|false {
    if ($x) {
        return 1;
    }
    return false;
};
