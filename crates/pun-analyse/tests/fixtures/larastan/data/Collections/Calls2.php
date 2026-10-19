<?php

namespace App\PunCollections;

use Illuminate\Support\Collection;

final class Point
{
}

enum Suit
{
    case Hearts;
}

class Calls2
{
    /**
     * @param  Collection<int, Point>  $points
     * @param  Collection<int, Suit>  $suits
     * @param  Collection<int, \stdClass>  $objects
     * @param  Collection<int, mixed>  $mixed
     */
    public function run(Collection $points, Collection $suits, Collection $objects, \Illuminate\Database\Query\Builder $db, Collection $mixed): void
    {
        $points->toArray();
        $suits->toArray();
        $objects->toArray();
        Writer::everyone()->count();
        Writer::query()->{'get'}()->count();
        Writer::pluck('name')->sum();
        Writer::pluck('nope')->sum();
        $db->get()->max('id');
        $objects->max('id');
        $mixed->toArray();
    }
}
