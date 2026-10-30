<?php

namespace Probe;

/**
 * @param list<int|void> $withVoid
 * @param array{}|list<int> $emptyOrList
 * @param list{int, ...<string>} $unsealedList
 */
function run(array $withVoid, array $emptyOrList, array $unsealedList)
{
    array_map(function ($v) { return $v; }, $withVoid);
    array_map(function ($v) { return $v; }, $emptyOrList);
    array_map(function ($v) { return $v; }, $unsealedList);
}
-----
<?php

namespace Probe;

/**
 * @param list<int|void> $withVoid
 * @param array{}|list<int> $emptyOrList
 * @param list{int, ...<string>} $unsealedList
 */
function run(array $withVoid, array $emptyOrList, array $unsealedList)
{
    array_map(function ($v) { return $v; }, $withVoid);
    array_map(function (int $v) { return $v; }, $emptyOrList);
    array_map(function ($v) { return $v; }, $unsealedList);
}
