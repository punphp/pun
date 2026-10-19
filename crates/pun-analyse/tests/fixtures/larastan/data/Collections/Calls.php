<?php

namespace App\PunCollections;

use Illuminate\Database\Eloquent\Builder;

class Calls
{
    /**
     * @param  Builder<Writer>  $query
     */
    public function run(Writer $writer, Builder $query): void
    {
        Writer::all()->count();
        Writer::all()->pluck('name');
        Writer::all()->pluck('nope');
        Writer::all()->pluck('email');
        Writer::all()->first();
        Writer::all()->first(fn ($w) => true);
        Writer::query()->get()->count();
        Writer::query()->get()->isEmpty();
        Writer::query()->get()->contains($writer);
        Writer::query()->get()->contains(fn ($w) => true);
        Writer::query()->get()->contains('name', 'x');
        Writer::query()->get()->contains('nope', 'x');
        Writer::query()->get()->where('name', 'x');
        Writer::query()->get()->where('nope', 'x');
        Writer::query()->get()->max();
        Writer::query()->pluck('name')->sum();
        Writer::query()->pluck('nope')->sum();
        Writer::query()->get()->map(fn ($w) => $w)->count();
        $query->get()->take(2);
        $writer->friends()->get()->count();
        Writer::all()->Count();
        Writer::all()->modelKeys();
        Writer::all()->firstWhere('name', 'x');
        Writer::all()->avg('id');
        Writer::all()->sum('id');
        Writer::all()->containsStrict('name', 'x');
        Writer::all()->containsStrict($writer);
        collect([1])->count();
        $all = Writer::all();
        $all->count();
    }
}
