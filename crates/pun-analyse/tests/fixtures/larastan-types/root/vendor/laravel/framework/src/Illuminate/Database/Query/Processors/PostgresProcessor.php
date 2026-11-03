<?php

namespace Illuminate\Database\Query\Processors;

use Illuminate\Database\Query\Builder;

class PostgresProcessor extends Processor
{
    /**
     * Process an "insert get ID" query.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  string  $sql
     * @param  array  $values
     * @param  string|null  $sequence
     * @return int
     */
    public function processInsertGetId(Builder $query, $sql, $values, $sequence = null)
    {}

    /** @inheritDoc */
    public function processTypes($results)
    {}

    /** @inheritDoc */
    public function processColumns($results)
    {}

    /** @inheritDoc */
    public function processIndexes($results)
    {}

    /** @inheritDoc */
    public function processForeignKeys($results)
    {}
}
