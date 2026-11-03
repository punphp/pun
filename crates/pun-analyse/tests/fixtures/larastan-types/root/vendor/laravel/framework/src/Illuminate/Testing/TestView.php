<?php

namespace Illuminate\Testing;

use Closure;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Testing\Assert as PHPUnit;
use Illuminate\Testing\Constraints\SeeInHtml;
use Illuminate\Testing\Constraints\SeeInOrder;
use Illuminate\View\View;
use Stringable;

class TestView implements Stringable
{
    use Macroable;

    /**
     * The original view.
     *
     * @var \Illuminate\View\View
     */
    protected $view;

    /**
     * The rendered view contents.
     *
     * @var string
     */
    protected $rendered;

    /**
     * Create a new test view instance.
     *
     * @param  \Illuminate\View\View  $view
     */
    public function __construct(View $view)
    {}

    /**
     * Assert that the response view has a given piece of bound data.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function assertViewHas($key, $value = null)
    {}

    /**
     * Assert that the response view has a given list of bound data.
     *
     * @param  array  $bindings
     * @return $this
     */
    public function assertViewHasAll(array $bindings)
    {}

    /**
     * Assert that the response view is missing a piece of bound data.
     *
     * @param  string  $key
     * @return $this
     */
    public function assertViewMissing($key)
    {}

    /**
     * Assert that the view's rendered content is empty.
     *
     * @return $this
     */
    public function assertViewEmpty()
    {}

    /**
     * Assert that the given string or array of strings are contained within the view.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertSee($value, $escape = true)
    {}

    /**
     * Assert that the given HTML string or array of HTML strings are contained within the view.
     *
     * @param  string|list<string>  $value
     * @return $this
     */
    public function assertSeeHtml($value)
    {}

    /**
     * Assert that the given strings are contained in order within the view.
     *
     * @param  list<string>  $values
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeInOrder(array $values, $escape = true)
    {}

    /**
     * Assert that the given HTML strings are contained in order within the view.
     *
     * @param  list<string>  $values
     * @return $this
     */
    public function assertSeeHtmlInOrder(array $values)
    {}

    /**
     * Assert that the given string or array of strings are contained within the view text.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeText($value, $escape = true)
    {}

    /**
     * Assert that the given strings are contained in order within the view text.
     *
     * @param  list<string>  $values
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeTextInOrder(array $values, $escape = true)
    {}

    /**
     * Assert that the given string or array of strings are not contained within the view.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSee($value, $escape = true)
    {}

    /**
     * Assert that the given HTML string or array of HTML strings are not contained within the view.
     *
     * @param  string|list<string>  $value
     * @return $this
     */
    public function assertDontSeeHtml($value)
    {}

    /**
     * Assert that the given string or array of strings are not contained within the view text.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSeeText($value, $escape = true)
    {}

    /**
     * Get the string contents of the rendered view.
     *
     * @return string
     */
    public function __toString()
    {}
}
