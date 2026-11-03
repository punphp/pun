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

class VerificationExpectation extends Expectation
{
    #[Override]
    public function __clone()
    {}

    /**
     * @return void
     */
    public function clearCountValidators()
    {}
}
