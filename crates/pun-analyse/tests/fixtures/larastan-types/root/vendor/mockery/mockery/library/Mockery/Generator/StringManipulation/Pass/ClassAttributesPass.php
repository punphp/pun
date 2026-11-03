<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator\StringManipulation\Pass;

use Mockery\Exception;
use Mockery\Generator\MockConfiguration;
use Mockery\Generator\TargetClassInterface;
use Override;

use function implode;
use function str_replace;

class ClassAttributesPass implements Pass
{
    /**
     * @param  string $code
     * @return string
     *
     * @throws Exception
     */
    #[Override]
    public function apply($code, MockConfiguration $config)
    {}
}
