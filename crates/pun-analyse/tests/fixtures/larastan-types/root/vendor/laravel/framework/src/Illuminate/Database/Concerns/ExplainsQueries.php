<?php

namespace Illuminate\Database\Concerns;

use Illuminate\Support\Collection;

trait ExplainsQueries
{
    /**
     * Explains the query.
     *
     * @return \Illuminate\Support\Collection
     */
    public function explain()
    {}
}
