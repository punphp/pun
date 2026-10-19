<?php

namespace App\PunRelations;

use Illuminate\Database\Eloquent\Builder;

class Calls
{
    /**
     * @param  Builder<Author>  $authors
     * @param  'books'|'foo'  $either
     * @param  array{books: string, 0: 'foo'}  $shape
     */
    public function run(Builder $authors, Author $author, BaseModel $base, mixed $unknown, string $name, string $either, array $shape): void
    {
        Author::query()->has('books');
        Author::query()->has('foo');
        Author::query()->orHas('foo');
        Author::query()->doesntHave('foo');
        Author::query()->orDoesntHave('foo');
        Author::query()->whereHas('foo');
        Author::query()->withWhereHas('foo');
        Author::query()->orWhereHas('foo');
        Author::query()->whereDoesntHave('foo');
        Author::query()->orWhereDoesntHave('foo');
        Author::query()->whereRelation('foo', 'id', 1);
        Author::query()->hasNot('foo');
        Author::query()->Has('foo');
        Author::with('foo');
        Author::with('books');
        Author::with('books:id,title');
        Author::with('foo:id');
        Author::with(['books', 'foo', 'bar']);
        Author::with(['foo' => fn ($q) => $q, 'latestBook']);
        Author::with('books.chapters');
        Author::with('books.foo');
        Author::with('foo.books');
        Author::with(['foo', 'books.foo', 'bar']);
        Author::with(['books.chapters', 'foo']);
        Author::with(['foo', 'books.chapters']);
        Author::with('untyped');
        Author::with('name');
        Author::with('latestBook.chapters.foo');
        $authors->with('foo');
        $authors->whereHas('books');
        $author->books()->has('foo');
        $author->books()->has('author');
        $author->latestBook()->with('foo');
        $author->with('foo');
        $base->with('foo');
        Chapter::with('foo');
        BaseModel::with('foo');
        $unknown->has('foo');
        Author::with($name);
        Author::with($either);
        Author::with($shape);
        Author::with();
        $model = new Author();
        $model::with('foo');
        Book::query()->whereHas('chapters.foo');
        Book::query()->with('author.books.foo');
    }
}
