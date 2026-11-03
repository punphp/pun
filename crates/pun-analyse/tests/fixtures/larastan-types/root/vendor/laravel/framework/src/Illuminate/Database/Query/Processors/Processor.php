<?php

namespace Illuminate\Database\Query\Processors;

use Illuminate\Database\Query\Builder;

class Processor
{
    /**
     * Process the results of a "select" query.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  array  $results
     * @return array
     */
    public function processSelect(Builder $query, $results)
    {}

    /**
     * Process an  "insert get ID" query.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  string  $sql
     * @param  array  $values
     * @param  string|null  $sequence
     * @return int
     */
    public function processInsertGetId(Builder $query, $sql, $values, $sequence = null)
    {}

    /**
     * Process the results of a schemas query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string, path: string|null, default: bool}>
     */
    public function processSchemas($results)
    {}

    /**
     * Process the results of a tables query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string, schema: string|null, schema_qualified_name: string, size: int|null, comment: string|null, collation: string|null, engine: string|null}>
     */
    public function processTables($results)
    {}

    /**
     * Process the results of a views query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string, schema: string|null, schema_qualified_name: string, definition: string}>
     */
    public function processViews($results)
    {}

    /**
     * Process the results of a types query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string, schema: string, schema_qualified_name: string, type: string, category: string, implicit: bool}>
     */
    public function processTypes($results)
    {}

    /**
     * Process the results of a columns query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string, type: string, type_name: string, collation: string|null, nullable: bool, default: mixed, auto_increment: bool, comment: string|null, generation: array{type: string|null, expression: string|null}|null}>
     */
    public function processColumns($results)
    {}

    /**
     * Process the results of an indexes query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string, columns: list<string>, type: string|null, unique: bool, primary: bool}>
     */
    public function processIndexes($results)
    {}

    /**
     * Process the results of a foreign keys query.
     *
     * @param  list<array<string, mixed>>  $results
     * @return list<array{name: string|null, columns: list<string>, foreign_schema: string|null, foreign_table: string, foreign_columns: list<string>, on_update: string|null, on_delete: string|null}>
     */
    public function processForeignKeys($results)
    {}
}
