<?php

namespace Illuminate\Database\Concerns;

use Illuminate\Container\Container;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\MultipleRecordsFoundException;
use Illuminate\Database\Query\Expression;
use Illuminate\Database\RecordNotFoundException;
use Illuminate\Database\RecordsNotFoundException;
use Illuminate\Pagination\Cursor;
use Illuminate\Pagination\CursorPaginator;
use Illuminate\Pagination\LengthAwarePaginator;
use Illuminate\Pagination\Paginator;
use Illuminate\Support\Collection;
use Illuminate\Support\LazyCollection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use InvalidArgumentException;
use RuntimeException;
use SortDirection;

/**
 * @template TValue
 *
 * @mixin \Illuminate\Database\Query\Builder
 */
trait BuildsQueries
{
    use Conditionable;

    /**
     * Chunk the results of the query.
     *
     * @param  int  $count
     * @param  callable(\Illuminate\Support\Collection<int, TValue>, int): mixed  $callback
     * @return bool
     */
    public function chunk($count, callable $callback)
    {}

    /**
     * Run a map over each item while chunking.
     *
     * @template TReturn
     *
     * @param  callable(TValue): TReturn  $callback
     * @param  int  $count
     * @return \Illuminate\Support\Collection<int, TReturn>
     */
    public function chunkMap(callable $callback, $count = 1000)
    {}

    /**
     * Execute a callback over each item while chunking.
     *
     * @param  callable(TValue, int): mixed  $callback
     * @param  int  $count
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function each(callable $callback, $count = 1000)
    {}

    /**
     * Chunk the results of a query by comparing IDs.
     *
     * @param  int  $count
     * @param  callable(\Illuminate\Support\Collection<int, TValue>, int): mixed  $callback
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return bool
     */
    public function chunkById($count, callable $callback, $column = null, $alias = null)
    {}

    /**
     * Chunk the results of a query by comparing IDs in descending order.
     *
     * @param  int  $count
     * @param  callable(\Illuminate\Support\Collection<int, TValue>, int): mixed  $callback
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return bool
     */
    public function chunkByIdDesc($count, callable $callback, $column = null, $alias = null)
    {}

    /**
     * Chunk the results of a query by comparing IDs in a given order.
     *
     * @param  int  $count
     * @param  callable(\Illuminate\Support\Collection<int, TValue>, int): mixed  $callback
     * @param  string|null  $column
     * @param  string|null  $alias
     * @param  SortDirection|bool  $descending
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function orderedChunkById($count, callable $callback, $column = null, $alias = null, $descending = false)
    {}

    /**
     * Execute a callback over each item while chunking by ID.
     *
     * @param  callable(TValue, int): mixed  $callback
     * @param  int  $count
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return bool
     */
    public function eachById(callable $callback, $count = 1000, $column = null, $alias = null)
    {}

    /**
     * Query lazily, by chunks of the given size.
     *
     * @param  int  $chunkSize
     * @return \Illuminate\Support\LazyCollection<int, TValue>
     *
     * @throws \InvalidArgumentException
     */
    public function lazy($chunkSize = 1000)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return \Illuminate\Support\LazyCollection<int, TValue>
     *
     * @throws \InvalidArgumentException
     */
    public function lazyById($chunkSize = 1000, $column = null, $alias = null)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs in descending order.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return \Illuminate\Support\LazyCollection<int, TValue>
     *
     * @throws \InvalidArgumentException
     */
    public function lazyByIdDesc($chunkSize = 1000, $column = null, $alias = null)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs in a given order.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @param  SortDirection|bool  $descending
     * @return \Illuminate\Support\LazyCollection
     *
     * @throws \InvalidArgumentException
     * @throws \RuntimeException
     */
    protected function orderedLazyById($chunkSize = 1000, $column = null, $alias = null, $descending = false)
    {}

    /**
     * Get the ID of the last result of a chunk to continue paging from.
     *
     * @param  mixed  $result
     * @param  string  $alias
     * @return mixed
     */
    protected function getLastIdFromChunk($result, $alias)
    {}

    /**
     * Execute the query and get the first result.
     *
     * @param  array|string  $columns
     * @return TValue|null
     */
    public function first($columns = ['*'])
    {}

    /**
     * Execute the query and get the first result or throw an exception.
     *
     * @param  array|string  $columns
     * @param  string|null  $message
     * @return TValue
     *
     * @throws \Illuminate\Database\RecordNotFoundException
     */
    public function firstOrFail($columns = ['*'], $message = null)
    {}

    /**
     * Execute the query and get the first result if it's the sole matching record.
     *
     * @param  array|string  $columns
     * @return TValue
     *
     * @throws \Illuminate\Database\RecordsNotFoundException
     * @throws \Illuminate\Database\MultipleRecordsFoundException
     */
    public function sole($columns = ['*'])
    {}

    /**
     * Paginate the given query using a cursor paginator.
     *
     * @param  int  $perPage
     * @param  array|string  $columns
     * @param  string  $cursorName
     * @param  \Illuminate\Pagination\Cursor|string|null  $cursor
     * @return \Illuminate\Contracts\Pagination\CursorPaginator
     */
    protected function paginateUsingCursor($perPage, $columns = ['*'], $cursorName = 'cursor', $cursor = null)
    {}

    /**
     * Get the original column name of the given column, without any aliasing.
     *
     * @param  \Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $builder
     * @param  string  $parameter
     * @return string
     */
    protected function getOriginalColumnNameForCursorPagination($builder, string $parameter)
    {}

    /**
     * Create a new length-aware paginator instance.
     *
     * @param  \Illuminate\Support\Collection  $items
     * @param  int  $total
     * @param  int  $perPage
     * @param  int  $currentPage
     * @param  array  $options
     * @return \Illuminate\Pagination\LengthAwarePaginator
     */
    protected function paginator($items, $total, $perPage, $currentPage, $options)
    {}

    /**
     * Create a new simple paginator instance.
     *
     * @param  \Illuminate\Support\Collection  $items
     * @param  int  $perPage
     * @param  int  $currentPage
     * @param  array  $options
     * @return \Illuminate\Pagination\Paginator
     */
    protected function simplePaginator($items, $perPage, $currentPage, $options)
    {}

    /**
     * Create a new cursor paginator instance.
     *
     * @param  \Illuminate\Support\Collection  $items
     * @param  int  $perPage
     * @param  \Illuminate\Pagination\Cursor  $cursor
     * @param  array  $options
     * @return \Illuminate\Pagination\CursorPaginator
     */
    protected function cursorPaginator($items, $perPage, $cursor, $options)
    {}

    /**
     * Pass the query to a given callback and then return it.
     *
     * @param  callable($this): mixed  $callback
     * @return $this
     */
    public function tap($callback)
    {}

    /**
     * Pass the query to a given callback and return the result.
     *
     * @template TReturn
     *
     * @param  (callable($this): TReturn)  $callback
     * @return (TReturn is null|void ? $this : TReturn)
     */
    public function pipe($callback)
    {}
}
