<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

use Mockery;
use Mockery\Exception\NoMatchingExpectationException;
use Throwable;

use const PHP_EOL;

use function array_pop;
use function array_unshift;

use function end;

class ExpectationDirector
{
    /**
     * Stores an array of all default expectations for this mock
     *
     * @var list<Expectation>
     */
    protected $_defaults = [];

    /**
     * Stores an array of all expectations for this mock
     *
     * @var list<Expectation>
     */
    protected $_expectations = [];

    /**
     * The expected order of next call
     *
     * @var int
     */
    protected $_expectedOrder;

    /**
     * Mock object the director is attached to
     *
     * @var LegacyMockInterface|MockInterface
     */
    protected $_mock;

    /**
     * Method name the director is directing
     *
     * @var string
     */
    protected $_name;

    /**
     * @param string $name
     */
    public function __construct($name, LegacyMockInterface $mock)
    {}

    /**
     * Add a new expectation to the director
     */
    public function addExpectation(Expectation $expectation)
    {}

    /**
     * Handle a method call being directed by this instance
     *
     * @return mixed
     *
     * @throws Throwable
     */
    public function call(array $args)
    {}

    /**
     * Attempt to locate an expectation matching the provided args
     *
     * @return null|Expectation
     */
    public function findExpectation(array $args)
    {}

    /**
     * Return all expectations assigned to this director
     *
     * @return array<Expectation>
     */
    public function getDefaultExpectations()
    {}

    /**
     * Return the number of expectations assigned to this director.
     *
     * @return int
     */
    public function getExpectationCount()
    {}

    /**
     * Return all expectations assigned to this director
     *
     * @return array<Expectation>
     */
    public function getExpectations()
    {}

    /**
     * Make the given expectation a default for all others assuming it was correctly created last
     *
     * @return void
     *
     * @throws Exception
     */
    public function makeExpectationDefault(Expectation $expectation)
    {}

    /**
     * Verify all expectations of the director
     *
     * @return void
     *
     * @throws Exception
     */
    public function verify()
    {}

    /**
     * Search current array of expectations for a match
     *
     * @param  array<Expectation> $expectations
     * @return null|Expectation
     */
    protected function _findExpectationIn(array $expectations, array $args)
    {}
}
