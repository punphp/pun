<?php

declare(strict_types=1);

namespace App\Models;

use App\Casts\InboundCast;
use App\Casts\MoneyCast;
use App\Casts\PlainClass;
use App\Casts\Status;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;

final class Everything extends Model
{
    use SoftDeletes;

    protected $casts = [
        'int_col' => 'string',
        'json_col' => 'array',
        'bool_col' => 'boolean',
        'decimal_col' => 'decimal:2',
        'text_col' => 'encrypted:object',
        'point_col' => Status::class,
        'float_col' => MoneyCast::class,
        'enum_col' => 'immutable_datetime',
        'set_col' => 'collection',
        'year_col' => 'timestamp',
        'uint_col' => InboundCast::class,
        'added_col' => 'real',
        'unknown_col' => 'App\\Casts\\Missing',
        'nullable_one' => PlainClass::class,
        'nullable_false' => 'date',
        'taggable_type' => 'integer',
        'nullable_const' => 'json',
        'unsigned_int_col' => 'float',
        'nullable_taggable_id' => 'double',
        'a list item',
    ];

    protected $withoutDefault;

    public function publicCamel(): string
    {
        return '';
    }

    public function getLegacyAttrAttribute(): string
    {
        return '';
    }

    /** @return Attribute<string, never> */
    protected function upperAccessor(): Attribute
    {
        return Attribute::make(get: fn (): string => '');
    }

    protected function notAccessor(): int
    {
        return 1;
    }

    /** @return array<string, string> */
    protected function casts(): array
    {
        return [
            'in_if' => 'boolean',
            'after_col' => Status::class,
            'renamed' => 'int',
        ];
    }
}
