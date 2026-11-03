<?php

namespace Illuminate\Testing;

use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Testing\Assert as PHPUnit;
use Illuminate\Testing\Constraints\SeeInHtml;
use Illuminate\Testing\Constraints\SeeInOrder;
use Stringable;

class TestComponent implements Stringable
{
    use Macroable {
        __call as macroCall;
    }

    /**
     * The original component.
     *
     * @var \Illuminate\View\Component
     */
    public $component;

    /**
     * The rendered component contents.
     *
     * @var string
     */
    protected $rendered;

    /**
     * Create a new test component instance.
     *
     * @param  \Illuminate\View\Component  $component
     * @param  \Illuminate\View\View  $view
     */
    public function __construct($component, $view)
    {}

    /**
     * Assert that the given string or array of strings are contained within the rendered component.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertSee($value, $escape = true)
    {}

    /**
     * Assert that the given HTML string or array of HTML strings are contained within the rendered component.
     *
     * @param  string|list<string>  $value
     * @return $this
     */
    public function assertSeeHtml($value)
    {}

    /**
     * Assert that the given strings are contained in order within the rendered component.
     *
     * @param  array  $values
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeInOrder(array $values, $escape = true)
    {}

    /**
     * Assert that the given HTML strings are contained in order within the rendered component.
     *
     * @param  list<string>  $values
     * @return $this
     */
    public function assertSeeHtmlInOrder(array $values)
    {}

    /**
     * Assert that the given string or array of strings are contained within the rendered component text.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeText($value, $escape = true)
    {}

    /**
     * Assert that the given strings are contained in order within the rendered component text.
     *
     * @param  list<string>  $values
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeTextInOrder(array $values, $escape = true)
    {}

    /**
     * Assert that the given string or array of strings are not contained within the rendered component.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSee($value, $escape = true)
    {}

    /**
     * Assert that the given HTML string or array of HTML strings are not contained within the rendered component.
     *
     * @param  string|list<string>  $value
     * @return $this
     */
    public function assertDontSeeHtml($value)
    {}

    /**
     * Assert that the given string or array of strings are not contained within the rendered component text.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSeeText($value, $escape = true)
    {}

    /**
     * Get the string contents of the rendered component.
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Dynamically access properties on the underlying component.
     *
     * @param  string  $attribute
     * @return mixed
     */
    public function __get($attribute)
    {}

    /**
     * Dynamically call methods on the underlying component.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
