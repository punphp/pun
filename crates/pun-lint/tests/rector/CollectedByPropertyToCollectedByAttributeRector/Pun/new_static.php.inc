<?php

use Illuminate\Database\Eloquent\Model;

class A extends Model
{
    public function newCollection(array $models = [])
    {
        return new static($models);
    }
}
