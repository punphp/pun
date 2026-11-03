<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

use Override;
use ReturnTypeWillChange;
use RuntimeException;

use function array_map;
use function current;
use function implode;
use function reset;

class CompositeExpectation implements ExpectationInterface
{
    /**
     * Stores an array of all expectations for this composite
     *
     * @var array<Expectation>
     */
    protected $_expectations = [];

    /**
     * Intercept any expectation calls and direct against all expectations
     *
     * @param  string       $method
     * @param  array<mixed> $args
     * @return self
     */
    public function __call($method, array $args)
    {}

    /**
     * Return the string summary of this composite expectation
     *
     * @return string
     */
    #[ReturnTypeWillChange]
    public function __toString()
    {}

    /**
     * Add an expectation to the composite
     *
     * @param  Expectation $expectation
     * @return void
     */
    public function add($expectation)
    {}

    /**
     * @param  mixed ...$args
     * @return self
     */
    #[Override]
    public function andReturn(...$args)
    {}

    /**
     * Set a return value, or sequential queue of return values
     *
     * @param  mixed ...$args
     * @return self
     */
    #[Override]
    public function andReturns(...$args)
    {}

    /**
     * Return the parent mock of the first expectation
     *
     * @return MockInterface
     *
     * @throws RuntimeException
     */
    #[Override]
    public function getMock()
    {}

    /**
     * Return order number of the first expectation
     *
     * @return int
     *
     * @throws RuntimeException
     */
    #[Override]
    public function getOrderNumber()
    {}

    /**
     * Mockery API alias to getMock
     *
     * @return MockInterface
     *
     * @throws RuntimeException
     */
    public function mock()
    {}

    /**
     * Starts a new expectation addition on the first mock which is the primary target outside of a demeter chain
     *
     * @param  mixed                                                ...$args
     * @return ($args is list{} ? HigherOrderMessage : Expectation)
     *
     * @throws RuntimeException
     */
    public function shouldNotReceive(...$args)
    {}

    /**
     * Starts a new expectation addition on the first mock which is the primary target, outside of a demeter chain
     *
     * @param  mixed                                                ...$args
     * @return ($args is list{} ? HigherOrderMessage : Expectation)
     *
     * @throws RuntimeException
     */
    public function shouldReceive(...$args)
    {}

    /**
     * Return the first expectation
     *
     * @return Expectation
     *
     * @throws RuntimeException If no expectations have been added to this composite expectation
     */
    private function firstExpectation()
    {}
}
