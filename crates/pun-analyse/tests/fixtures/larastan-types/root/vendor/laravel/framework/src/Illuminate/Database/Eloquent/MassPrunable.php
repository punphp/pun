<?php

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Events\ModelsPruned;
use LogicException;

trait MassPrunable
{
    /**
     * Prune all prunable models in the database.
     *
     * @param  int  $chunkSize
     * @return int
     */
    public function pruneAll(int $chunkSize = 1000)
    {}

    /**
     * Get the prunable model query.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     *
     * @throws \LogicException
     */
    public function prunable()
    {}
}
