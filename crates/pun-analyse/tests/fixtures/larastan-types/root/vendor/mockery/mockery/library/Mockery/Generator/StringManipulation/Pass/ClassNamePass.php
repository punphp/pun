<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator\StringManipulation\Pass;

use Mockery\Generator\MockConfiguration;
use Override;

use function ltrim;
use function str_replace;

class ClassNamePass implements Pass
{
    /**
     * @param  string $code
     * @return string
     */
    #[Override]
    public function apply($code, MockConfiguration $config)
    {}
}
