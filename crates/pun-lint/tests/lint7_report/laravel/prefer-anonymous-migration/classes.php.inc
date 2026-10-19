<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Migrations as M;

class CreateUsersTable extends Migration
{
    public function up(): void
    {
    }
}

final class CreatePostsTable extends \Illuminate\Database\Migrations\Migration
{
}

abstract class Lower extends \illuminate\database\migrations\MIGRATION
{
}

class Aliased extends M\Migration
{
}

class Other extends BaseController
{
}

class Plain
{
}

return new class extends Migration {
    public function up(): void
    {
    }
};
-----
6:1: Use anonymous classes for migrations instead of named classes.
13:1: Use anonymous classes for migrations instead of named classes.
17:1: Use anonymous classes for migrations instead of named classes.
21:1: Use anonymous classes for migrations instead of named classes.
