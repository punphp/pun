<?php

namespace Illuminate\Foundation\Testing\Concerns;

use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Events\QueryExecuted;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\DB;
use Illuminate\Testing\Constraints\CountInDatabase;
use Illuminate\Testing\Constraints\HasInDatabase;
use Illuminate\Testing\Constraints\NotSoftDeletedInDatabase;
use Illuminate\Testing\Constraints\SoftDeletedInDatabase;
use PHPUnit\Framework\Constraint\LogicalNot as ReverseConstraint;

trait InteractsWithDatabase
{
    /**
     * Assert that a given where condition exists in the database.
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  array<string, mixed>  $data
     * @param  string|null  $connection
     * @return $this
     */
    protected function assertDatabaseHas($table, array $data = [], $connection = null)
    {}

    /**
     * Assert that a given where condition does not exist in the database.
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  array<string, mixed>  $data
     * @param  string|null  $connection
     * @return $this
     */
    protected function assertDatabaseMissing($table, array $data = [], $connection = null)
    {}

    /**
     * Assert the count of table entries.
     *
     * @param  iterable<class-string<\Illuminate\Database\Eloquent\Model>|string, int>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  int|null  $count
     * @param  string|null  $connection
     * @return $this
     */
    protected function assertDatabaseCount($table, ?int $count = null, $connection = null)
    {}

    /**
     * Assert that the given table or tables has no entries.
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  string|null  $connection
     * @return $this
     */
    protected function assertDatabaseEmpty($table, $connection = null)
    {}

    /**
     * Assert the given record has been "soft deleted".
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  array<string, mixed>  $data
     * @param  string|null  $connection
     * @param  string|null  $deletedAtColumn
     * @return $this
     */
    protected function assertSoftDeleted($table, array $data = [], $connection = null, $deletedAtColumn = 'deleted_at')
    {}

    /**
     * Assert the given record has not been "soft deleted".
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  array<string, mixed>  $data
     * @param  string|null  $connection
     * @param  string|null  $deletedAtColumn
     * @return $this
     */
    protected function assertNotSoftDeleted($table, array $data = [], $connection = null, $deletedAtColumn = 'deleted_at')
    {}

    /**
     * Assert the given model exists in the database.
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $model
     * @return $this
     */
    protected function assertModelExists($model)
    {}

    /**
     * Assert the given model does not exist in the database.
     *
     * @param  iterable<\Illuminate\Database\Eloquent\Model>|\Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $model
     * @return $this
     */
    protected function assertModelMissing($model)
    {}

    /**
     * Specify the number of database queries that should occur throughout the test.
     *
     * @param  int  $expected
     * @param  string|null  $connection
     * @return $this
     */
    public function expectsDatabaseQueryCount($expected, $connection = null)
    {}

    /**
     * Determine if the argument is a soft deletable model.
     *
     * @param  mixed  $model
     * @return bool
     */
    protected function isSoftDeletableModel($model)
    {}

    /**
     * Cast a JSON string to a database compatible type.
     *
     * @param  array|object|string  $value
     * @param  string|null  $connection
     * @return \Illuminate\Contracts\Database\Query\Expression
     */
    public function castAsJson($value, $connection = null)
    {}

    /**
     * Get the database connection.
     *
     * @param  string|null  $connection
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string|null  $table
     * @return \Illuminate\Database\Connection
     */
    protected function getConnection($connection = null, $table = null)
    {}

    /**
     * Get the table name from the given model or string.
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @return string
     */
    protected function getTable($table)
    {}

    /**
     * Get the table connection specified in the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @return string|null
     */
    protected function getTableConnection($table)
    {}

    /**
     * Get the table column name used for soft deletes.
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @param  string  $defaultColumnName
     * @return string
     */
    protected function getDeletedAtColumn($table, $defaultColumnName = 'deleted_at')
    {}

    /**
     * Get the model entity from the given model or string.
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>|string  $table
     * @return \Illuminate\Database\Eloquent\Model|null
     */
    protected function newModelFor($table)
    {}

    /**
     * Seed a given database connection.
     *
     * @param  list<string>|class-string<\Illuminate\Database\Seeder>|string  $class
     * @return $this
     */
    public function seed($class = 'Database\\Seeders\\DatabaseSeeder')
    {}
}
