<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\CountValidator;

use Mockery\Exception\InvalidCountException;
use Override;

use const PHP_EOL;

class Exact extends CountValidatorAbstract
{
    /**
     * Validate the call count against this validator
     *
     * @param  int  $n
     * @return bool
     *
     * @throws InvalidCountException
     */
    #[Override]
    public function validate($n)
    {}
}
