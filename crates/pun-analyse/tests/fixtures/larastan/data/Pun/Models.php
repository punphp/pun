<?php

namespace Pun\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
    public function scopeTyped(string $query)
    {
    }

    public function scopeOther(NotAModel $query)
    {
    }

    public function scopeUntyped($query)
    {
    }

    public function scopePublished(Builder $query)
    {
    }

    protected function scopeHidden(Builder $query)
    {
    }

    public function plain()
    {
    }

    public function name(): Attribute
    {
        return Attribute::make();
    }

    public function maybe(): Attribute|int
    {
        return 1;
    }

    public static function build()
    {
        self::make();
        static::make();
        parent::make();
    }
}

class Draft extends Post
{
    public function scopePublished(Builder $query)
    {
    }
}

class NotAModel
{
    public function scopePublished(Builder $query)
    {
    }
}
