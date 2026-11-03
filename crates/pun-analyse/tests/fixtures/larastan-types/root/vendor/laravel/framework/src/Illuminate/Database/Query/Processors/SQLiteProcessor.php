<?php

namespace Illuminate\Database\Query\Processors;

class SQLiteProcessor extends Processor
{
    /** @inheritDoc */
    public function processColumns($results, $sql = '')
    {}

    /** @inheritDoc */
    public function processIndexes($results)
    {}

    /** @inheritDoc */
    public function processForeignKeys($results)
    {}
}
