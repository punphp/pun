<?php

$a = function () {
    echo 1;
};

$b = function ($x) use ($a) {
    echo $x;
};

$c = function (): int {
    return 1;
};
-----
<?php

$a = function (): void {
    echo 1;
};

$b = function ($x) use ($a): void {
    echo $x;
};

$c = function (): int {
    return 1;
};
