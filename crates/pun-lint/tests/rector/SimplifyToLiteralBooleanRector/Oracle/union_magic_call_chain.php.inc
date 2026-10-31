<?php

final class Post
{
    public string $title = '';

    public function body(): string
    {
        return '';
    }
}

/**
 * @template T
 */
final class Box
{
    /**
     * @return T
     */
    public function sole(): mixed
    {
        return null;
    }
}

final class Model
{
    /**
     * @return Box<Post>
     */
    public function posts(): Box
    {
        return new Box();
    }

    public function name(): string
    {
        return '';
    }
}

final class Models
{
    public function __call(string $method, array $arguments): mixed
    {
        return null;
    }
}

it('reads the method of the members that have it down a chain', function (Model|Models $model, Models $models): void {
    $post = $model->posts()->sole();
    expect($post->body())->toBe('');
    expect($post->title)->toBe('');
    expect($model->posts()->sole()->body())->toBe('');
    $name = $model->name();
    expect($name)->toBe('');
    expect($model?->name())->toBe('');
    expect($model?->posts()->sole()->title)->toBe('');
    $magic = $models->posts();
    expect($magic)->toBe([]);
});
-----
<?php

final class Post
{
    public string $title = '';

    public function body(): string
    {
        return '';
    }
}

/**
 * @template T
 */
final class Box
{
    /**
     * @return T
     */
    public function sole(): mixed
    {
        return null;
    }
}

final class Model
{
    /**
     * @return Box<Post>
     */
    public function posts(): Box
    {
        return new Box();
    }

    public function name(): string
    {
        return '';
    }
}

final class Models
{
    public function __call(string $method, array $arguments): mixed
    {
        return null;
    }
}

it('reads the method of the members that have it down a chain', function (Model|Models $model, Models $models): void {
    $post = $model->posts()->sole();
    expect($post->body())->toBeEmpty();
    expect($post->title)->toBeEmpty();
    expect($model->posts()->sole()->body())->toBeEmpty();
    $name = $model->name();
    expect($name)->toBeEmpty();
    expect($model?->name())->toBeEmpty();
    expect($model?->posts()->sole()->title)->toBeEmpty();
    $magic = $models->posts();
    expect($magic)->toBe([]);
});
