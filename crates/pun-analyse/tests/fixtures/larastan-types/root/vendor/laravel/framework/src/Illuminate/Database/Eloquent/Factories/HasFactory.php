<?php

namespace Illuminate\Database\Eloquent\Factories;

use Illuminate\Database\Eloquent\Attributes\UseFactory;

/**
 * @template TFactory of \Illuminate\Database\Eloquent\Factories\Factory
 */
trait HasFactory
{
    /**
     * Get a new factory instance for the model.
     *
     * @param  (callable(array<string, mixed>, static|null): array<string, mixed>)|array<string, mixed>|int|null  $count
     * @param  (callable(array<string, mixed>, static|null): array<string, mixed>)|array<string, mixed>  $state
     * @return TFactory
     */
    public static function factory($count = null, $state = [])
    {}

    /**
     * Create a new factory instance for the model.
     *
     * @return TFactory|null
     */
    protected static function newFactory()
    {}

    /**
     * Get the factory from the UseFactory class attribute.
     *
     * @return TFactory|null
     */
    protected static function getUseFactoryAttribute()
    {}
}
