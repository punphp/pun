<?php

namespace Illuminate\Testing;

use ArrayAccess;
use Closure;
use Countable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Testing\Assert as PHPUnit;
use JsonSerializable;

use function Illuminate\Support\enum_value;

class AssertableJsonString implements ArrayAccess, Countable
{
    /**
     * The original encoded json.
     *
     * @var \Illuminate\Contracts\Support\Jsonable|\JsonSerializable|array|string
     */
    public $json;

    /**
     * The decoded json contents.
     *
     * @var array|null
     */
    protected $decoded;

    /**
     * Create a new assertable JSON string instance.
     *
     * @param  \Illuminate\Contracts\Support\Jsonable|\JsonSerializable|array|string  $jsonable
     */
    public function __construct($jsonable)
    {}

    /**
     * Validate and return the decoded response JSON.
     *
     * @param  string|null  $key
     * @return mixed
     */
    public function json($key = null)
    {}

    /**
     * Assert that the response JSON has the expected count of items at the given key.
     *
     * @param  int  $count
     * @param  string|null  $key
     * @return $this
     */
    public function assertCount(int $count, $key = null)
    {}

    /**
     * Assert that the response has the exact given JSON.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertExact(array $data)
    {}

    /**
     * Assert that the response has the similar JSON as given.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertSimilar(array $data)
    {}

    /**
     * Assert that the response contains the given JSON fragment.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertFragment(array $data)
    {}

    /**
     * Assert that the response does not contain the given JSON fragment.
     *
     * @param  array  $data
     * @param  bool  $exact
     * @return $this
     */
    public function assertMissing(array $data, $exact = false)
    {}

    /**
     * Assert that the response does not contain the exact JSON fragment.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertMissingExact(array $data)
    {}

    /**
     * Assert that the response does not contain the given path.
     *
     * @param  string  $path
     * @return $this
     */
    public function assertMissingPath($path)
    {}

    /**
     * Assert that the expected value and type exists at the given path in the response.
     *
     * @param  string  $path
     * @param  mixed  $expect
     * @return $this
     */
    public function assertPath($path, $expect)
    {}

    /**
     * Assert that the given path in the response contains all of the expected values without looking at the order.
     *
     * @param  string  $path
     * @param  array  $expect
     * @return $this
     */
    public function assertPathCanonicalizing($path, $expect)
    {}

    /**
     * Assert that the response has a given JSON structure.
     *
     * @param  array|null  $structure
     * @param  array|null  $responseData
     * @param  bool  $exact
     * @return $this
     */
    public function assertStructure(?array $structure = null, $responseData = null, bool $exact = false)
    {}

    /**
     * Assert that the response is a superset of the given JSON.
     *
     * @param  array  $data
     * @param  bool  $strict
     * @return $this
     */
    public function assertSubset(array $data, $strict = false)
    {}

    /**
     * Reorder associative array keys to make it easy to compare arrays.
     *
     * @param  array  $data
     * @return array
     */
    protected function reorderAssocKeys(array $data)
    {}

    /**
     * Get the assertion message for assertJson.
     *
     * @param  array  $data
     * @return string
     */
    protected function assertJsonMessage(array $data)
    {}

    /**
     * Get the strings we need to search for when examining the JSON.
     *
     * @param  string  $key
     * @param  string  $value
     * @return array
     */
    protected function jsonSearchStrings($key, $value)
    {}

    /**
     * Get the total number of items in the underlying JSON array.
     *
     * @return int
     */
    public function count(): int
    {}

    /**
     * Determine whether an offset exists.
     *
     * @param  mixed  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value at the given offset.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at the given offset.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value at the given offset.
     *
     * @param  string  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}
}
