<?php

namespace Probe;

class Node
{
    /**
     * @return list<static>
     */
    public static function many(): array
    {
        return [];
    }

    public static function mapMany()
    {
        return array_map(function ($node) {
            return $node;
        }, static::many());
    }

    public function mapThis()
    {
        return array_map(function ($node) {
            return $node;
        }, [$this, $this]);
    }
}

/**
 * @param non-empty-array<int>|list<string> $nonEmptyOrList
 * @param non-empty-array<int>|non-empty-list<string> $bothNonEmpty
 * @param array<int>|non-empty-list<float> $listNonEmpty
 */
function run(array $nonEmptyOrList, array $bothNonEmpty, array $listNonEmpty)
{
    array_map(function ($v) { return $v; }, $nonEmptyOrList);
    array_map(function ($v) { return $v; }, $bothNonEmpty);
    array_map(function ($v) { return $v; }, $listNonEmpty);
}
-----
<?php

namespace Probe;

class Node
{
    /**
     * @return list<static>
     */
    public static function many(): array
    {
        return [];
    }

    public static function mapMany()
    {
        return array_map(function (self $node) {
            return $node;
        }, static::many());
    }

    public function mapThis()
    {
        return array_map(function ($node) {
            return $node;
        }, [$this, $this]);
    }
}

/**
 * @param non-empty-array<int>|list<string> $nonEmptyOrList
 * @param non-empty-array<int>|non-empty-list<string> $bothNonEmpty
 * @param array<int>|non-empty-list<float> $listNonEmpty
 */
function run(array $nonEmptyOrList, array $bothNonEmpty, array $listNonEmpty)
{
    array_map(function (int|string $v) { return $v; }, $nonEmptyOrList);
    array_map(function ($v) { return $v; }, $bothNonEmpty);
    array_map(function (int|float $v) { return $v; }, $listNonEmpty);
}
