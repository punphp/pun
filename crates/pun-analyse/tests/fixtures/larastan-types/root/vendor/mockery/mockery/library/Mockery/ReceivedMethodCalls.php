<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

class ReceivedMethodCalls
{
    private $methodCalls = [];

    public function push(MethodCall $methodCall)
    {}

    public function verify(Expectation $expectation)
    {}
}
