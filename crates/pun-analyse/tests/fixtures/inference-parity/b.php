<?php

declare(strict_types=1);

namespace App;

/**
 * @phpstan-type Row array{id: int, name: string}
 */
final class Src
{
    /** @return Row */
    public function row(): array
    {
        return ['id' => 1, 'name' => 'a'];
    }
}

/**
 * @phpstan-import-type Row from Src
 * @phpstan-import-type Row from Src as Other
 * @phpstan-type Mine array{a: int}
 */
final class User2
{
    public function go(Src $s, mixed $m): int
    {
        /** @var Row $r */
        $r = $s->row();
        /** @var Other $o */
        $o = $m;
        /** @var Mine $mine */
        $mine = $m;
        /** @var list<Row> $rows */
        $rows = $m;
        $f = function () use ($m): int {
            /** @var Row $inner */
            $inner = $m;

            return $inner['id'];
        };

        return $r['id'] + $o['id'] + $mine['a'] + count($rows) + $f();
    }
}
