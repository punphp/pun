<?php

namespace Illuminate\Database\Schema;

use Illuminate\Database\Connection;
use Illuminate\Database\Query\Expression;
use Illuminate\Support\Collection;
use Illuminate\Support\Fluent;
use Illuminate\Support\Str;

class BlueprintState
{
    /**
     * The blueprint instance.
     *
     * @var \Illuminate\Database\Schema\Blueprint
     */
    protected $blueprint;

    /**
     * The connection instance.
     *
     * @var \Illuminate\Database\Connection
     */
    protected $connection;

    /**
     * The columns.
     *
     * @var \Illuminate\Database\Schema\ColumnDefinition[]
     */
    private $columns;

    /**
     * The primary key.
     *
     * @var \Illuminate\Database\Schema\IndexDefinition|null
     */
    private $primaryKey;

    /**
     * The indexes.
     *
     * @var \Illuminate\Database\Schema\IndexDefinition[]
     */
    private $indexes;

    /**
     * The foreign keys.
     *
     * @var \Illuminate\Database\Schema\ForeignKeyDefinition[]
     */
    private $foreignKeys;

    /**
     * Create a new blueprint state instance.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Database\Connection  $connection
     */
    public function __construct(Blueprint $blueprint, Connection $connection)
    {}

    /**
     * Get the primary key.
     *
     * @return \Illuminate\Database\Schema\IndexDefinition|null
     */
    public function getPrimaryKey()
    {}

    /**
     * Get the columns.
     *
     * @return \Illuminate\Database\Schema\ColumnDefinition[]
     */
    public function getColumns()
    {}

    /**
     * Get the indexes.
     *
     * @return \Illuminate\Database\Schema\IndexDefinition[]
     */
    public function getIndexes()
    {}

    /**
     * Get the foreign keys.
     *
     * @return \Illuminate\Database\Schema\ForeignKeyDefinition[]
     */
    public function getForeignKeys()
    {}

    /**
     * Update the blueprint's state.
     *
     * @param  \Illuminate\Support\Fluent  $command
     * @return void
     */
    public function update(Fluent $command)
    {}
}
