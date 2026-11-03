<?php

namespace Orchestra\Sidekick\Eloquent;

use BackedEnum;
use Carbon\CarbonInterface;
use Illuminate\Database\Eloquent\Concerns\HasUlids;
use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Concerns\AsPivot;
use Illuminate\Database\Eloquent\Relations\Pivot;
use Illuminate\Support\Arr;
use InvalidArgumentException;
use JsonSerializable;
use Orchestra\Sidekick\SensitiveValue;
use Stringable;
use Throwable;

if (! \function_exists('Orchestra\Sidekick\Eloquent\column_name')) {
    /**
     * Get qualify column name from Eloquent model.
     *
     * @api
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>  $model
     *
     * @throws \InvalidArgumentException
     */
    function column_name(Model|string $model, string $attribute): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\is_pivot_model')) {
    /**
     * Determine if the given model is a pivot model.
     *
     * @api
     *
     * @template TPivotModel of (\Illuminate\Database\Eloquent\Model&\Illuminate\Database\Eloquent\Relations\Concerns\AsPivot)|\Illuminate\Database\Eloquent\Relations\Pivot
     *
     * @param  TPivotModel|class-string<TPivotModel>  $model
     *
     * @throws \InvalidArgumentException
     */
    function is_pivot_model(Pivot|Model|string $model): bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\model_exists')) {
    /**
     * Check whether given $model exists.
     *
     * @api
     */
    function model_exists(mixed $model): bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\model_from')) {
    /**
     * Check whether given $model exists.
     *
     * @api
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>  $model
     * @param  array<string, mixed>  $attributes
     */
    function model_from(Model|string $model, array $attributes): Model
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\model_key_type')) {
    /**
     * Check whether given $model key type.
     *
     * @api
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>  $model
     *
     * @throws \InvalidArgumentException
     */
    function model_key_type(Model|string $model): string
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\model_diff')) {
    /**
     * Get attributes diff state from a model.
     *
     * @api
     *
     * @param  array<int, string>  $excludes
     * @return array<string, mixed>
     */
    function model_diff(Model $model, array $excludes = [], bool $withTimestamps = true): array
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\model_snapshot')) {
    /**
     * Store a snapshot for a model and return the original attributes.
     *
     * @api
     *
     * @return array<string, mixed>|null
     */
    function model_snapshot(Model $model): ?array
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\model_state')) {
    /**
     * Get attributes original and changed state from a model.
     *
     * @api
     *
     * @param  array<int, string>  $excludes
     * @return array{0: array<string, mixed>|null, 1: array<string, mixed>}
     */
    function model_state(Model $model, array $excludes = [], bool $withTimestamps = true): array
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\normalize_value')) {
    /**
     * Normalize the given value to be store to database as scalar.
     *
     * @api
     *
     * @return scalar
     */
    function normalize_value(mixed $value): mixed
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\summarize_changes')) {
    /**
     * Get table name from Eloquent model.
     *
     * @api
     *
     * @param  array<string, mixed>  $changes
     * @param  array<int, string>  $hiddens
     * @return array<string, \Orchestra\Sidekick\SensitiveValue<mixed>|scalar>
     */
    function summarize_changes(array $changes, array $hiddens = []): array
    {}
}

if (! \function_exists('Orchestra\Sidekick\Eloquent\table_name')) {
    /**
     * Get table name from Eloquent model.
     *
     * @api
     *
     * @param  \Illuminate\Database\Eloquent\Model|class-string<\Illuminate\Database\Eloquent\Model>  $model
     *
     * @throws \InvalidArgumentException
     */
    function table_name(Model|string $model): string
    {}
}
