<?php

namespace Pun\Appends;

use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

/**
 * @property int $tagged
 */
class Widget extends Model
{
    protected $appends = array(
        'id',
        'tagged',
        'label' => 'title',
        'legacy',
        1,
        "interpolated",
        ...self::MORE,
        'computed',
    );

    const MORE = ['more'];

    public $others = ['nope'], $more = ['nope'];

    public function getLegacyAttribute()
    {
        return 1;
    }

    protected function computed(): Attribute
    {
        return Attribute::make();
    }
}

/**
 * @property string $id
 */
class Tagged extends Model
{
    protected $appends = ['id', 'missing'];
}

abstract class Base extends Model
{
    protected $appends = ['id'];
}

class Child extends Base
{
    protected $primaryKey = 'uuid';

    protected $appends = ['uuid', 'id'];
}

class Plain
{
    protected $appends = ['nope'];
}

class Hooked
{
    public $untyped;

    public array $hooked = ['hooked'] {
        get => $this->hooked;
    }
}

class Computed extends Model
{
    protected $appends = self::APPENDS;

    const APPENDS = ['nope'];
}
