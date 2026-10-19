<?php

namespace App\PunRelations;

use Illuminate\Database\Eloquent\Model;

abstract class BaseModel extends Model
{
}

class Chapter extends BaseModel
{
}

class NotAModel
{
    public static function with($relations)
    {
    }
}
