<?php

namespace Illuminate\Support\Testing\Fakes;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection;

trait MatchesProperties
{
    /**
     * @param  callable|array<string, mixed>|null  $callback
     * @return callable|null
     */
    protected function resolveTruthTest($callback)
    {}
}
