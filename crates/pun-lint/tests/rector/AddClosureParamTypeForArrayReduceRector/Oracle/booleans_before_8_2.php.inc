<?php

namespace Probe;

/**
 * @param list<int|true> $intOrTrue
 * @param list<true> $trues
 * @param list<false|null> $falseOrNull
 * @param list<int|false|null> $falsy
 */
function run(array $intOrTrue, array $trues, array $falseOrNull, array $falsy)
{
    array_reduce($intOrTrue, function ($carry, $item) {
        return 1;
    }, 0);
    array_reduce($trues, function ($carry, $item) {
        return 1;
    }, 0);
    array_reduce($falseOrNull, function ($carry, $item) {
        return 1;
    }, 0);
    array_reduce($falsy, function ($carry, $item) {
        return 1;
    }, 0);
}
-----
<?php

namespace Probe;

/**
 * @param list<int|true> $intOrTrue
 * @param list<true> $trues
 * @param list<false|null> $falseOrNull
 * @param list<int|false|null> $falsy
 */
function run(array $intOrTrue, array $trues, array $falseOrNull, array $falsy)
{
    array_reduce($intOrTrue, function (int $carry, int|bool $item) {
        return 1;
    }, 0);
    array_reduce($trues, function (int $carry, bool $item) {
        return 1;
    }, 0);
    array_reduce($falseOrNull, function (int $carry, ?bool $item) {
        return 1;
    }, 0);
    array_reduce($falsy, function (int $carry, int|false|null $item) {
        return 1;
    }, 0);
}
