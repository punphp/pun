<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// Tables named after models without `$table`: Str::snake(Str::pluralStudly(class_basename(...))).
return new class extends Migration
{
    public function up(): void
    {
        Schema::create('categories', function (Blueprint $table): void {
            $table->id();
            $table->string('position');
        });
        Schema::create('boxes', function (Blueprint $table): void {
            $table->id();
            $table->string('label');
        });
        Schema::create('stats', function (Blueprint $table): void {
            $table->id();
            $table->integer('total');
        });
    }
};
