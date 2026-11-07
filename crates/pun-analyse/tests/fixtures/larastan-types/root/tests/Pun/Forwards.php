<?php

namespace Pun\Forwards;

use App\User;

function forwards(): void
{
    // a query builder method through the Eloquent builder's mixin returns the Eloquent builder
    User::query()->limit(1)->get()->count();
    // passed through, but its `$this` is still the Eloquent builder (not the query builder, whose rows are stdClass)
    User::query()->dump()->get()->max('foo');
    // a model forwards what it lacks to its builder
    User::where('email', 'x')->get()->count();
    User::firstOrFail()->accounts()->get()->count();
    User::first()->group()->has('foo');
}
