<?php declare(strict_types=1);

namespace MaxTraitInheritDoc;

class Model {}

/** @template TModel of Model */
class Factory
{
    /**
     * @param  array<string, mixed>|callable(array<string, mixed>): array<string, mixed>  $attributes
     * @param  TModel|null  $parent
     * @return TModel
     */
    public function create($attributes = [], $parent = null)
    {
        throw new \LogicException();
    }
}

// a trait method's {@inheritDoc} takes the overridden method's @param types in the using class's context
trait RefreshOnCreate
{
    /**
     * {@inheritDoc}
     */
    public function create($attributes = [], $parent = null)
    {
        \PHPStan\dumpType($attributes);
        \PHPStan\dumpType($parent);
        throw new \LogicException();
    }
}

/** @extends Factory<Model> */
final class UserFactory extends Factory
{
    use RefreshOnCreate;
}
