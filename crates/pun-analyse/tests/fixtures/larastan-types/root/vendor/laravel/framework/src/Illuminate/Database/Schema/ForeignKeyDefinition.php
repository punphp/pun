<?php

namespace Illuminate\Database\Schema;

use Illuminate\Support\Fluent;

/**
 * @method ForeignKeyDefinition deferrable(bool $value = true) Set the foreign key as deferrable (PostgreSQL)
 * @method ForeignKeyDefinition initiallyImmediate(bool $value = true) Set the default time to check the constraint (PostgreSQL)
 * @method ForeignKeyDefinition inplace(bool $value = true) Specify that algorithm=inplace should be used for the foreign key operation, which requires foreign key checks to be disabled (MySQL)
 * @method ForeignKeyDefinition lock(('none'|'shared'|'default'|'exclusive') $value) Specify the DDL lock mode for the foreign key operation (MySQL)
 * @method ForeignKeyDefinition on(string $table) Specify the referenced table
 * @method ForeignKeyDefinition onDelete(string $action) Add an ON DELETE action
 * @method ForeignKeyDefinition onUpdate(string $action) Add an ON UPDATE action
 * @method ForeignKeyDefinition references(string|string[] $columns) Specify the referenced column(s)
 */
class ForeignKeyDefinition extends Fluent
{
    /**
     * Indicate that updates should cascade.
     *
     * @return $this
     */
    public function cascadeOnUpdate()
    {}

    /**
     * Indicate that updates should be restricted.
     *
     * @return $this
     */
    public function restrictOnUpdate()
    {}

    /**
     * Indicate that updates should set the foreign key value to null.
     *
     * @return $this
     */
    public function nullOnUpdate()
    {}

    /**
     * Indicate that updates should have "no action".
     *
     * @return $this
     */
    public function noActionOnUpdate()
    {}

    /**
     * Indicate that deletes should cascade.
     *
     * @return $this
     */
    public function cascadeOnDelete()
    {}

    /**
     * Indicate that deletes should be restricted.
     *
     * @return $this
     */
    public function restrictOnDelete()
    {}

    /**
     * Indicate that deletes should set the foreign key value to null.
     *
     * @return $this
     */
    public function nullOnDelete()
    {}

    /**
     * Indicate that deletes should have "no action".
     *
     * @return $this
     */
    public function noActionOnDelete()
    {}
}
