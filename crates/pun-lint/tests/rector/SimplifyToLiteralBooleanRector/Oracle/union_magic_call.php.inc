<?php

final class Model
{
    public function name(): string
    {
        return '';
    }

    public function tags(): array
    {
        return [];
    }
}

final class Models
{
    public function __call(string $method, array $arguments): mixed
    {
        return null;
    }
}

final class Nothing
{
}

final class Fluent
{
    public function copy(): static
    {
        return $this;
    }
}

it('reads the method of the members that have it', function (Model|Models $model, Model|Nothing $other, Models $models): void {
    expect($model->name())->toBe('');
    expect($model->tags())->toBe([]);
    expect($other->name())->toBe('');
    expect($models->name())->toBe('');
});

it('keeps the type it does not know', function (Fluent|Models $fluent, Model|Models $model, string $method, Model|int $either): void {
    expect($fluent->copy())->toBe([]);
    expect($model->$method())->toBe('');
    expect($either->name())->toBe('');
});
-----
<?php

final class Model
{
    public function name(): string
    {
        return '';
    }

    public function tags(): array
    {
        return [];
    }
}

final class Models
{
    public function __call(string $method, array $arguments): mixed
    {
        return null;
    }
}

final class Nothing
{
}

final class Fluent
{
    public function copy(): static
    {
        return $this;
    }
}

it('reads the method of the members that have it', function (Model|Models $model, Model|Nothing $other, Models $models): void {
    expect($model->name())->toBeEmpty();
    expect($model->tags())->toBeEmpty();
    expect($other->name())->toBeEmpty();
    expect($models->name())->toBe('');
});

it('keeps the type it does not know', function (Fluent|Models $fluent, Model|Models $model, string $method, Model|int $either): void {
    expect($fluent->copy())->toBe([]);
    expect($model->$method())->toBe('');
    expect($either->name())->toBeEmpty();
});
