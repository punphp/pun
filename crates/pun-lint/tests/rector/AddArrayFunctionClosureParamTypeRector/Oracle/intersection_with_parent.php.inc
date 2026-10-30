<?php

namespace Probe;

abstract class Model
{
    public $name;
}

class Stub extends Model
{
}

interface Named
{
}

/**
 * @template TModel of Model
 */
class Builder
{
    /**
     * @return array<int, TModel>
     */
    public function getModels()
    {
        return [];
    }
}

final class T
{
    /**
     * @param  array<int, Stub&Model>  $models
     * @param  array<int, Stub&Named>  $named
     * @param  Builder<Model>  $builder
     */
    public function run(array $models, array $named, Builder $builder)
    {
        array_map(fn ($model) => $model->name, $models);
        array_map(fn ($model) => $model->name, $named);
        $items = $builder->getModels();
        assert(is_iterable($items));
        array_map(fn ($model) => $model->name, $items);
    }
}
-----
<?php

namespace Probe;

abstract class Model
{
    public $name;
}

class Stub extends Model
{
}

interface Named
{
}

/**
 * @template TModel of Model
 */
class Builder
{
    /**
     * @return array<int, TModel>
     */
    public function getModels()
    {
        return [];
    }
}

final class T
{
    /**
     * @param  array<int, Stub&Model>  $models
     * @param  array<int, Stub&Named>  $named
     * @param  Builder<Model>  $builder
     */
    public function run(array $models, array $named, Builder $builder)
    {
        array_map(fn (\Probe\Stub $model) => $model->name, $models);
        array_map(fn (\Probe\Stub&\Probe\Named $model) => $model->name, $named);
        $items = $builder->getModels();
        assert(is_iterable($items));
        array_map(fn (\Probe\Model $model) => $model->name, $items);
    }
}
