<?php
namespace App;
use function PHPStan\dumpType;
class A {
    /**
     * @param array<string, int> $x
     * @param list<string> $l
     * @param array<int, bool> $bools
     * @param non-empty-list<string> $nel
     */
    public function f(array $x, array $l, array $bools, array $nel, string $s, int $i): void {
        dumpType(array_flip($x));
        dumpType(array_flip($l));
        dumpType(array_flip($nel));
        dumpType(array_flip(['a', 'b', 'a']));
        dumpType(array_flip(['x' => 1, 'y' => '2', 'z' => '02']));
        dumpType(array_flip([$s, 'b']));
        dumpType(array_flip([1 => 'a', 0 => 'b']));
        dumpType(array_flip(['a' => true, 'b' => null]));
        dumpType(array_intersect_key($l, $nel));
        dumpType(array_intersect_key($l, $x));
        dumpType(array_intersect_key($x, $l));
        dumpType(array_intersect_key($x, ['a' => 1]));
        dumpType(array_intersect_key(['a' => 1, 'b' => 2], $x, ['a' => 1]));
        dumpType(array_intersect_key(['a' => 1, 'b' => 2], ['a' => 1], ['b' => 1]));
        dumpType(array_intersect_key(['a' => 1, 'b' => 2]));
        dumpType(array_intersect_key([1, 2, 3], [0 => 1, 1 => 2]));
        dumpType(array_intersect_key($bools, ['a' => 1]));
    }
}
