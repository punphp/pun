<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

class VerificationDirector
{
    /**
     * @var ReceivedMethodCalls
     */
    private $receivedMethodCalls;

    /**
     * @var VerificationExpectation
     */
    private $verificationExpectation;

    public function __construct(ReceivedMethodCalls $receivedMethodCalls, VerificationExpectation $expectation)
    {}

    /**
     * @return self
     */
    public function atLeast()
    {}

    /**
     * @return self
     */
    public function atMost()
    {}

    /**
     * @param  int  $minimum
     * @param  int  $maximum
     * @return self
     */
    public function between($minimum, $maximum)
    {}

    /**
     * @return self
     */
    public function once()
    {}

    /**
     * @param  int|null $limit
     * @return self
     */
    public function times($limit = null)
    {}

    /**
     * @return self
     */
    public function twice()
    {}

    /**
     * @return void
     */
    public function verify()
    {}

    /**
     * @param  mixed $args
     * @return self
     */
    public function with(...$args)
    {}

    /**
     * @return self
     */
    public function withAnyArgs()
    {}

    /**
     * @param  mixed $args
     * @return self
     */
    public function withArgs($args)
    {}

    /**
     * @return self
     */
    public function withNoArgs()
    {}

    /**
     * @param  string       $method
     * @param  array<mixed> $args
     * @return self
     */
    protected function cloneApplyAndVerify($method, $args)
    {}

    /**
     * @param  string       $method
     * @param  array<mixed> $args
     * @return self
     */
    protected function cloneWithoutCountValidatorsApplyAndVerify($method, $args)
    {}
}
