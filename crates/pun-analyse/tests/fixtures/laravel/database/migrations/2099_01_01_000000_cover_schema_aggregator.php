<?php

declare(strict_types=1);

use App\Models\Link;
use App\Models\NoTable;
use App\Models\User;
use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

// Every branch of Larastan's SchemaAggregator, for crates/pun-analyse/tests/laravel.rs.
abstract class AbstractMigration extends Migration
{
    protected $connection = 'sqlite';

    abstract public function up(): void;
}

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('everythings', function (Blueprint $table): void {
            $table->id();
            $table->integer('int_col');
            $table->bigInteger('unsigned_int_col')->unsigned();
            $table->unsignedInteger('uint_col')->nullable();
            $table->string('nullable_false')->nullable(false);
            $table->string('nullable_const')->nullable(PHP_EOL);
            $table->string('nullable_one')->nullable(1);
            $table->string('nullable_upper')->nullable(TRUE);
            $table->string('nullable_named')->nullable(value: true);
            $table->text('text_col');
            $table->boolean('bool_col');
            $table->point('point_col');
            $table->decimal('decimal_col');
            $table->float('float_col');
            $table->enum('enum_col', ['draft', 'published']);
            $table->enum('enum_empty', []);
            $table->enum('status', ['draft', 'published']);
            $table->set('set_col', array('a', 'b', 'c' => 'c', $other, ...$more));
            $table->json('json_col');
            $table->vector('unknown_col');
            $table->addColumn('integer', 'added_col');
            $table->addColumn($type, $name);
            $table->morphs('taggable');
            $table->nullableMorphs('nullable_taggable');
            $table->nullableUuidMorphs('uuid_nullable_taggable');
            $table->uuidMorphs('uuid_taggable');
            $table->softDeletes();
            $table->year('year_col');
            $table->rememberToken();
            $table->ipAddress();
            $table->foreignIdFor(User::class);
            $table->foreignIdFor('App\Models\Link', 'link_ref')->unsigned();
            $table->foreignIdFor(NoTable::class);
            $table->foreignIdFor(NoTable::class, 'no_table_unsigned')->unsigned();
            $table->foreignIdFor($model);
            $table->timestamps();
            $table->index(['int_col']);
            $table->foreign('uint_col')->references('id')->on('users');
            $other->string('not_the_blueprint');
            Blueprint::make()->string('not_a_variable');
            $table->{$method}()->nullable();
            $unused = 1;
            $table->{$method}('dynamic_method');
            $table->string('chained')->{$method}();
            if ($condition) {
                $table->string('in_if');
            } else {
                $table->string('in_else');
            }
            $table->string('to_rename');
            $table->string('to_drop');
            $table->string('to_drop_array');
            $table->string('to_remove');
            $table->string('to_drop_if_exists');
            $table->morphs('dropped_morph');
            $table->string('public_camel');
            $table->integer('legacy_attr');
            $table->integer('upper_accessor');
            $table->string('not_accessor');
            $table->after('id', function (Blueprint $t): void {
                $t->string('after_col');
                return;
            });
            $table->after('id');
            return;
        });

        Schema::table('everythings', function ($table): void {
            $table->renameColumn('to_rename', 'renamed');
            $table->renameColumn('to_rename', $name);
            $table->renameColumn('missing_column', 'other');
            $table->dropColumn('to_drop');
            $table->dropColumn(['to_drop_array', $name]);
            $table->removeColumn('to_remove');
            $table->dropIfExists('to_drop_if_exists');
            $table->dropMorphs('dropped_morph');
            $table->timestampsTz();
            $table->nullableTimestampsTz();
            $table->dropRememberToken();
        });

        Schema::table('everythings', function (DB $table): void {
            $table->string('not_added');
        });
        Schema::table('everythings', 'not a closure');
        Schema::table('everythings', function (): void {});
        Schema::table($name, function (Blueprint $table): void {});
        Schema::table('missing_table', function (Blueprint $table): void {
            $table->string('x');
        });

        Schema::create('children', function (Blueprint $table): void {
            $table->uuid();
            $table->integer('score');
            $table->timestamps();
        });

        Schema::connection('sqlite')->create('second', function (Blueprint $table): void {
            $table->string('name');
        });
        Schema::rename('second', 'renamed_second');
        Schema::rename('nonexistent', 'other');
        Schema::rename($name, 'other');
        Schema::table('renamed_second', function (Blueprint $table): void {
            $table->rename('third');
            $table->string('after_rename');
            $table->rename($name);
        });

        Schema::create('dropped', function (Blueprint $table): void {});
        Schema::drop('dropped');
        Schema::dropIfExists('dropped_too');
        Schema::drop($name);
        Schema::hasTable('everythings');
        Schema::$property('x');
        Schema::connection('sqlite')->hasTable('x');
        Schema::connection('sqlite')->{$method}('x');
        DB::connection('sqlite')->create('not_schema', function (Blueprint $table): void {});
        DB::create('not_schema', function (Blueprint $table): void {});
        $schema::create('not_schema', function (Blueprint $table): void {});
        \Schema::create('global_alias', function (Blueprint $table): void {
            $table->string('name');
        });
    }

    public function down(): void
    {
        Schema::drop('everythings');
    }

    public function empty(): void
    {
    }
};
