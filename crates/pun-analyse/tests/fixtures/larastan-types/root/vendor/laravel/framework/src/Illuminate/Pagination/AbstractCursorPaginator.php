<?php

namespace Illuminate\Pagination;

use ArrayAccess;
use Closure;
use Exception;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Pivot;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\Tappable;
use Illuminate\Support\Traits\TransformsToResourceCollection;
use Stringable;
use Traversable;

/**
 * @template TKey of array-key
 *
 * @template-covariant TValue
 *
 * @mixin \Illuminate\Support\Collection<TKey, TValue>
 */
abstract class AbstractCursorPaginator implements Htmlable, Stringable
{
    use ForwardsCalls, Tappable, TransformsToResourceCollection;

    /**
     * All of the items being paginated.
     *
     * @var \Illuminate\Support\Collection<TKey, TValue>
     */
    protected $items;

    /**
     * The number of items to be shown per page.
     *
     * @var int
     */
    protected $perPage;

    /**
     * The base path to assign to all URLs.
     *
     * @var string
     */
    protected $path = '/';

    /**
     * The query parameters to add to all URLs.
     *
     * @var array
     */
    protected $query = [];

    /**
     * The URL fragment to add to all URLs.
     *
     * @var string|null
     */
    protected $fragment;

    /**
     * The cursor string variable used to store the page.
     *
     * @var string
     */
    protected $cursorName = 'cursor';

    /**
     * The current cursor.
     *
     * @var \Illuminate\Pagination\Cursor|null
     */
    protected $cursor;

    /**
     * The paginator parameters for the cursor.
     *
     * @var array
     */
    protected $parameters;

    /**
     * The paginator options.
     *
     * @var array
     */
    protected $options;

    /**
     * The current cursor resolver callback.
     *
     * @var \Closure|null
     */
    protected static $currentCursorResolver;

    /**
     * Get the URL for a given cursor.
     *
     * @param  \Illuminate\Pagination\Cursor|null  $cursor
     * @return string
     */
    public function url($cursor)
    {}

    /**
     * Get the URL for the previous page.
     *
     * @return string|null
     */
    public function previousPageUrl()
    {}

    /**
     * The URL for the next page, or null.
     *
     * @return string|null
     */
    public function nextPageUrl()
    {}

    /**
     * Get the "cursor" that points to the previous set of items.
     *
     * @return \Illuminate\Pagination\Cursor|null
     */
    public function previousCursor()
    {}

    /**
     * Get the "cursor" that points to the next set of items.
     *
     * @return \Illuminate\Pagination\Cursor|null
     */
    public function nextCursor()
    {}

    /**
     * Get a cursor instance for the given item.
     *
     * @param  \ArrayAccess|\stdClass  $item
     * @param  bool  $isNext
     * @return \Illuminate\Pagination\Cursor
     */
    public function getCursorForItem($item, $isNext = true)
    {}

    /**
     * Get the cursor parameters for a given object.
     *
     * @param  \ArrayAccess|\stdClass  $item
     * @return array
     *
     * @throws \Exception
     */
    public function getParametersForItem($item)
    {}

    /**
     * Get the cursor parameter value from a pivot model if applicable.
     *
     * @param  \ArrayAccess|\stdClass  $item
     * @param  string  $parameterName
     * @return string|null
     */
    protected function getPivotParameterForItem($item, $parameterName)
    {}

    /**
     * Ensure the parameter is a primitive type.
     *
     * This can resolve issues that arise the developer uses a value object for an attribute.
     *
     * @param  mixed  $parameter
     * @return mixed
     */
    protected function ensureParameterIsPrimitive($parameter)
    {}

    /**
     * Get / set the URL fragment to be appended to URLs.
     *
     * @param  string|null  $fragment
     * @return ($fragment is null ? string|null : $this)
     */
    public function fragment($fragment = null)
    {}

    /**
     * Add a set of query string values to the paginator.
     *
     * @param  array|string|null  $key
     * @param  string|null  $value
     * @return $this
     */
    public function appends($key, $value = null)
    {}

    /**
     * Add an array of query string values.
     *
     * @param  array  $keys
     * @return $this
     */
    protected function appendArray(array $keys)
    {}

    /**
     * Add all current query string values to the paginator.
     *
     * @return $this
     */
    public function withQueryString()
    {}

    /**
     * Add a query string value to the paginator.
     *
     * @param  string  $key
     * @param  string  $value
     * @return $this
     */
    protected function addQuery($key, $value)
    {}

    /**
     * Build the full fragment portion of a URL.
     *
     * @return string
     */
    protected function buildFragment()
    {}

    /**
     * Load a set of relationships onto the mixed relationship collection.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @return $this
     */
    public function loadMorph($relation, $relations)
    {}

    /**
     * Load a set of relationship counts onto the mixed relationship collection.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @return $this
     */
    public function loadMorphCount($relation, $relations)
    {}

    /**
     * Get the slice of items being paginated.
     *
     * @return array<TKey, TValue>
     */
    public function items()
    {}

    /**
     * Transform each item in the slice of items using a callback.
     *
     * @template TThroughValue
     *
     * @param  callable(TValue, TKey): TThroughValue  $callback
     * @return $this
     *
     * @phpstan-this-out static<TKey, TThroughValue>
     */
    public function through(callable $callback)
    {}

    /**
     * Get the number of items shown per page.
     *
     * @return int
     */
    public function perPage()
    {}

    /**
     * Get the current cursor being paginated.
     *
     * @return \Illuminate\Pagination\Cursor|null
     */
    public function cursor()
    {}

    /**
     * Get the query string variable used to store the cursor.
     *
     * @return string
     */
    public function getCursorName()
    {}

    /**
     * Set the query string variable used to store the cursor.
     *
     * @param  string  $name
     * @return $this
     */
    public function setCursorName($name)
    {}

    /**
     * Set the base path to assign to all URLs.
     *
     * @param  string  $path
     * @return $this
     */
    public function withPath($path)
    {}

    /**
     * Set the base path to assign to all URLs.
     *
     * @param  string  $path
     * @return $this
     */
    public function setPath($path)
    {}

    /**
     * Get the base path for paginator generated URLs.
     *
     * @return string|null
     */
    public function path()
    {}

    /**
     * Resolve the current cursor or return the default value.
     *
     * @param  string  $cursorName
     * @param  \Illuminate\Pagination\Cursor|null  $default
     * @return \Illuminate\Pagination\Cursor|null
     */
    public static function resolveCurrentCursor($cursorName = 'cursor', $default = null)
    {}

    /**
     * Set the current cursor resolver callback.
     *
     * @param  \Closure  $resolver
     * @return void
     */
    public static function currentCursorResolver(Closure $resolver)
    {}

    /**
     * Get an instance of the view factory from the resolver.
     *
     * @return \Illuminate\Contracts\View\Factory
     */
    public static function viewFactory()
    {}

    /**
     * Get an iterator for the items.
     *
     * @return \ArrayIterator<TKey, TValue>
     */
    public function getIterator(): Traversable
    {}

    /**
     * Determine if the list of items is empty.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the list of items is not empty.
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Get the number of items for the current page.
     *
     * @return int
     */
    public function count(): int
    {}

    /**
     * Get the paginator's underlying collection.
     *
     * @return \Illuminate\Support\Collection<TKey, TValue>
     */
    public function getCollection()
    {}

    /**
     * Set the paginator's underlying collection.
     *
     * @template TSetKey of array-key
     * @template TSetValue
     *
     * @param  \Illuminate\Support\Collection<TSetKey, TSetValue>  $collection
     * @return $this
     *
     * @phpstan-this-out static<TSetKey, TSetValue>
     */
    public function setCollection(Collection $collection)
    {}

    /**
     * Get the paginator options.
     *
     * @return array
     */
    public function getOptions()
    {}

    /**
     * Determine if the given item exists.
     *
     * @param  TKey  $key
     * @return bool
     */
    public function offsetExists($key): bool
    {}

    /**
     * Get the item at the given offset.
     *
     * @param  TKey  $key
     * @return TValue|null
     */
    public function offsetGet($key): mixed
    {}

    /**
     * Set the item at the given offset.
     *
     * @param  TKey|null  $key
     * @param  TValue  $value
     * @return void
     */
    public function offsetSet($key, $value): void
    {}

    /**
     * Unset the item at the given key.
     *
     * @param  TKey  $key
     * @return void
     */
    public function offsetUnset($key): void
    {}

    /**
     * Render the contents of the paginator to HTML.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Make dynamic calls into the collection.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}

    /**
     * Render the contents of the paginator when casting to a string.
     *
     * @return string
     */
    public function __toString()
    {}
}
